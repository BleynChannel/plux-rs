use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    vec,
};

use august_plugin_system::{
    context::LoadPluginContext,
    function::{Arg, DynamicFunction, FunctionOutput},
    utils::ManagerResult,
    variable::{Variable, VariableType},
    Api, Bundle, Manager, Plugin, Registry, Requests, StdInfo,
};
use mlua::{Function, IntoLua, Lua, MultiValue, Table, Value};
use semver::Version;

use crate::utils::load_config;

pub struct LuaPluginManager {
    lua_refs: HashMap<Bundle, Arc<Mutex<Lua>>>,
}

impl<'a> Manager<'a, FunctionOutput, StdInfo> for LuaPluginManager {
    fn format(&self) -> &'static str {
        "fpl"
    }

    fn load_plugin(
        &mut self,
        mut context: LoadPluginContext<'a, '_, FunctionOutput, StdInfo>,
        api: Api<FunctionOutput, StdInfo>,
    ) -> ManagerResult<()> {
        let bundle = context.plugin().info().bundle.clone();

        println!("FunctionPluginManager::load_plugin - {}", bundle);

        let lua = Arc::new(Mutex::new(Lua::new()));
        let api = Arc::new(api);

        {
            let lua = &*lua.lock().unwrap();

            self.registry_to_lua(lua, api.registry())?;
            self.register_api(lua, &api)?;
        }

        self.load_src(&lua, api, context.plugin().info().path.clone())?;

        let requests = self.register_requests(&lua, context.requests())?;
        for request in requests {
            context.register_request(request)?;
        }

        self.lua_refs.insert(bundle, lua);
        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &Plugin<'a, FunctionOutput, StdInfo>) -> ManagerResult<()> {
        let bundle = &plugin.info().bundle;

        println!("FunctionPluginManager::unload_plugin - {}", bundle);

        Ok(drop(self.lua_refs.remove(bundle)))
    }

    fn unregister_plugin(
        &mut self,
        plugin: &Plugin<'a, FunctionOutput, StdInfo>,
    ) -> ManagerResult<()> {
        let bundle = &plugin.info().bundle;

        println!("FunctionPluginManager::unregister_plugin - {}", bundle);

        Ok(())
    }

    fn register_plugin(
        &mut self,
        context: august_plugin_system::RegisterPluginContext,
    ) -> ManagerResult<StdInfo> {
        let (_, info) = load_config(context.path)?;

        println!(
            "FunctionPluginManager::register_plugin - {}",
            context.bundle
        );

        Ok(info)
    }
}

impl LuaPluginManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            lua_refs: HashMap::new(),
        }
    }

    // Добавление функций из реестра
    fn registry_to_lua(&self, lua: &Lua, registry: &Registry<FunctionOutput>) -> ManagerResult<()> {
        let globals = lua.globals();

        for function in registry.iter() {
            let function_name = function.name();
            let function = function.clone();
            let f = lua.create_function(move |ctx, lua_args: MultiValue| {
                let mut args = vec![];
                for arg in lua_args.iter().map(Self::lua2august) {
                    args.push(arg?);
                }

                let output = function
                    .call(&args)
                    .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?
                    .map(|var| Self::august2lua(&var, ctx));

                match output {
                    Some(out) => Ok(out?),
                    None => Ok(Value::Nil),
                }
            })?;

            globals.set(function_name, f)?;
        }

        Ok(())
    }

    // Загрузка исходного кода плагина
    fn load_src(
        &self,
        lua: &Arc<Mutex<Lua>>,
        api: Arc<Api<FunctionOutput, StdInfo>>,
        path: PathBuf,
    ) -> ManagerResult<()> {
        let arc_lua = lua.lock().unwrap();

        let src = std::fs::read_to_string(path.join("main.lua"))?;
        let result: Vec<Table> = arc_lua.load(&src).eval()?;

        let plugin = api.get_plugin_mut_by_bundle(api.plugin()).unwrap();
        let global = arc_lua.globals();

        for info in result.into_iter() {
            let name: String = info.get("name")?;
            let inputs: Vec<String> = info.get::<_, Vec<String>>("inputs")?;
            let function = info.get::<_, Function>("func")?;

            global.set(format!("__{}__", name), function)?;

            let lua = lua.clone();
            let function = DynamicFunction::new(
                name.clone(),
                inputs
                    .iter()
                    .map(|name| Arg::new(name, VariableType::Let))
                    .collect(),
                Some(Arg::new("output", VariableType::Let)),
                move |args| {
                    let arc_lua = lua.lock().unwrap();
                    let lua = &*arc_lua;

                    let mut lua_args = vec![];
                    for arg in args {
                        lua_args.push(Self::august2lua(arg, lua)?);
                    }

                    let f: mlua::Function = arc_lua.globals().get(format!("__{}__", name))?;

                    let result = match f.call::<_, Value>(MultiValue::from_vec(lua_args))? {
                        Value::Nil => Ok(None),
                        value => Ok(Some(Self::lua2august(&value)?)),
                    };
                    result
                },
            );

            plugin.register_function(function)?;
        }

        Ok(())
    }

    // Регистрация запрашиваемых функций
    fn register_requests(
        &self,
        lua: &Arc<Mutex<Lua>>,
        requests: &Requests,
    ) -> ManagerResult<Vec<DynamicFunction>> {
        let arc_lua = lua.lock().unwrap();

        let globals = arc_lua.globals();
        let mut result = vec![];

        for request in requests.iter() {
            match globals.get(request.name.clone())? {
                Value::Function(_) => {
                    let request_name = request.name.clone();
                    let lua = lua.clone();

                    let function = DynamicFunction::new(
                        request.name.clone(),
                        request
                            .inputs
                            .iter()
                            .enumerate()
                            .map(|(index, ty)| {
                                let str = format!("arg_{}", index);
                                Arg::new(str.as_str(), ty.clone())
                            })
                            .collect(),
                        request
                            .output
                            .map(|output| Arg::new("output", output.clone())),
                        move |args| {
                            let request_name = request_name.clone();

                            let arc_lua = lua.lock().unwrap();
                            let lua = &*arc_lua;

                            let mut lua_args = vec![];
                            for arg in args {
                                lua_args.push(Self::august2lua(arg, lua)?);
                            }

                            let f: mlua::Function = arc_lua.globals().get(request_name)?;

                            let result = match f.call::<_, Value>(MultiValue::from_vec(lua_args))? {
                                Value::Nil => Ok(None),
                                value => Ok(Some(Self::lua2august(&value)?)),
                            };
                            result
                        },
                    );

                    result.push(function);
                }
                Value::Nil => {
                    return Err(format!("Функции `{}` не существует", request.name).into())
                }
                _ => return Err(format!("`{}` должна быть функцией", request.name).into()),
            }
        }

        Ok(result)
    }

    // Регистрация API
    fn register_api<'a>(
        &self,
        lua: &Lua,
        api: &Arc<Api<FunctionOutput, StdInfo>>,
    ) -> ManagerResult<()> {
        let globals = lua.globals();

        {
            let api = api.clone();

            let f = lua.create_function(
                move |ctx, (id, version, name, args): (String, String, String, MultiValue)| {
                    let version = Version::parse(&version)
                        .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

                    let args = args
                        .iter()
                        .map(Self::lua2august)
                        .collect::<Result<Vec<_>, _>>()?;

                    let output = api
                        .call_function_depend(&id, &version, &name, args.as_slice())
                        .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?
                        .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?
                        .map(|var| Self::august2lua(&var, ctx));

                    match output {
                        Some(out) => Ok(out?),
                        None => Ok(Value::Nil),
                    }
                },
            )?;

            globals.set("call_function_depend", f)?;
        }

        {
            let api = api.clone();

            let f = lua.create_function(
                move |ctx, (id, version, name, args): (String, String, String, MultiValue)| {
                    let version = Version::parse(&version)
                        .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

                    let args = args
                        .iter()
                        .map(Self::lua2august)
                        .collect::<Result<Vec<_>, _>>()?;

                    let output = api
                        .call_function_optional_depend(&id, &version, &name, args.as_slice())
                        .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

                    match output {
                        Some(out) => Ok({
                            let output = out
                                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?
                                .map(|var| Self::august2lua(&var, ctx));

                            match output {
                                Some(out) => (true, out?),
                                None => (true, Value::Nil),
                            }
                        }),
                        None => Ok((false, Value::Nil)),
                    }
                },
            )?;

            globals.set("call_function_optional_depend", f)?;
        }

        Ok(())
    }

    fn lua2august(arg: &Value) -> mlua::Result<Variable> {
        match arg {
            Value::Nil => Ok(Variable::Null),
            Value::Boolean(var) => Ok(Variable::Bool(*var)),
            Value::LightUserData(_) => Err(mlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::Integer(var) => Ok(Variable::I32(*var as i32)),
            Value::Number(var) => Ok(Variable::F32(*var as f32)),
            Value::String(var) => Ok(Variable::String(var.to_str()?.to_string())),
            Value::Table(var) => {
                let mut list = vec![];
                for pair in var.clone().pairs::<Value, Value>() {
                    list.push(Self::lua2august(&pair?.1)?);
                }
                Ok(Variable::List(list))
            }
            Value::Function(_) => Err(mlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::Thread(_) => Err(mlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::UserData(_) => Err(mlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::Error(err) => Err(err.clone()),
        }
    }

    fn august2lua<'lua>(var: &Variable, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        match var {
            Variable::Null => Ok(Value::Nil),
            Variable::I8(var) => var.into_lua(lua),
            Variable::I16(var) => var.into_lua(lua),
            Variable::I32(var) => var.into_lua(lua),
            Variable::I64(var) => var.into_lua(lua),
            Variable::U8(var) => var.into_lua(lua),
            Variable::U16(var) => var.into_lua(lua),
            Variable::U32(var) => var.into_lua(lua),
            Variable::U64(var) => var.into_lua(lua),
            Variable::F32(var) => var.into_lua(lua),
            Variable::F64(var) => var.into_lua(lua),
            Variable::Bool(var) => var.into_lua(lua),
            Variable::Char(var) => var.to_string().into_lua(lua),
            Variable::String(var) => var.clone().into_lua(lua),
            Variable::List(var) => var
                .iter()
                .map(|v| Self::august2lua(v, lua))
                .collect::<mlua::Result<Vec<_>>>()?
                .into_lua(lua),
        }
    }
}
