use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};
use semver::Version;

use crate::{
    utils::{
        LoadPluginError, PluginCallRequestError, Ptr, RegisterManagerError, RegisterPluginError,
        StopLoaderError, UnloadPluginError, UnregisterManagerError, UnregisterPluginError,
    },
    variable::Variable,
    Bundle, Info, LoaderContext, Manager, Plugin, PluginInfo, Registry, Requests,
};

//TODO: Провести небольшой рефакторинг кода для комфортного использования библиотеки Rust-программистам
pub struct Loader<'a, O: Send + Sync, I: Info> {
    pub(crate) managers: Vec<Box<dyn Manager<'a, O, I>>>,
    pub(crate) registry: Registry<O>,
    pub(crate) requests: Requests,
    pub(crate) plugins: Vec<Plugin<'a, O, I>>,
}

impl<'a, O: Send + Sync, I: Info> Loader<'a, O, I> {
    pub const fn new() -> Self {
        Self {
            managers: vec![],
            registry: vec![],
            requests: vec![],
            plugins: vec![],
        }
    }

    pub fn context<FO, R>(&mut self, f: FO) -> R
    where
        FO: FnOnce(LoaderContext<'a, '_, O, I>) -> R,
    {
        f(LoaderContext::new(self))
    }

    pub fn stop(&mut self) -> Result<(), StopLoaderError> {
        private_loader::stop_plugins(self)?;
        private_loader::stop_managers(self)?;
        Ok(())
    }

    pub fn register_manager<M>(&mut self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'a, O, I> + 'static,
    {
        private_loader::register_manager(self, Box::new(manager))
    }

    pub unsafe fn forced_register_manager(
        &mut self,
        manager: Box<dyn Manager<'a, O, I>>,
    ) -> Result<(), RegisterManagerError> {
        private_loader::forced_register_manager(self, manager)
    }

    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a, O, I>>>,
    {
        managers
            .into_iter()
            .try_for_each(|manager| private_loader::register_manager(self, manager))?;

        Ok(())
    }

    pub fn par_register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoParallelIterator<Item = Box<dyn Manager<'a, O, I>>>,
    {
        let this = Ptr::new(self);
        managers.into_par_iter().try_for_each(move |manager| {
            private_loader::register_manager(this.as_mut(), manager)
        })?;

        Ok(())
    }

    pub fn unregister_manager(&mut self, format: &str) -> Result<(), UnregisterManagerError> {
        let index = self
            .managers
            .iter()
            .enumerate()
            .find_map(|(i, manager)| match manager.format() == format {
                true => Some(i),
                false => None,
            })
            .ok_or(UnregisterManagerError::NotFound)?;

        private_loader::unregister_manager(self, index)
    }

    pub unsafe fn forced_unregister_manager(
        &mut self,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        private_loader::forced_unregister_manager(&mut self.managers, index)
    }

	/*
	TODO: Пример рефакторинга: Добавить поиск менеджера по его типу
	*     Пример: let manager = loader.get_manager::<MyManager>();
	*/
    pub fn get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'a, O, I>>> {
        self.managers.iter().find(|m| m.format() == format)
    }

    pub fn par_get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'a, O, I>>> {
        self.managers
            .par_iter()
            .find_first(|m| m.format() == format)
    }

    pub fn get_manager_mut(&mut self, format: &str) -> Option<&mut Box<dyn Manager<'a, O, I>>> {
        self.managers.iter_mut().find(|m| m.format() == format)
    }

    pub fn par_get_manager_mut(&mut self, format: &str) -> Option<&mut Box<dyn Manager<'a, O, I>>> {
        self.managers
            .par_iter_mut()
            .find_first(|m| m.format() == format)
    }

	//TODO: Добавить параллельную версию
    pub fn register_plugin(&mut self, path: &str) -> Result<Bundle, RegisterPluginError> {
        private_loader::register_plugin(self, path)
    }

    pub unsafe fn forced_register_plugin(
        &mut self,
        manager: &mut Box<dyn Manager<'a, O, I>>,
        plugin_info: PluginInfo<I>,
    ) -> Result<Bundle, RegisterPluginError> {
        private_loader::forced_register_plugin(&mut self.plugins, Ptr::new(manager), plugin_info)
    }

	pub fn register_plugins<'b, P>(&mut self, paths: P) -> Result<Vec<Bundle>, RegisterPluginError>
    where
        P: IntoIterator<Item = &'b str>,
    {
        paths
            .into_iter()
            .map(|path| private_loader::register_plugin(self, path))
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn par_register_plugins<'b, P>(
        &mut self,
        paths: P,
    ) -> Result<Vec<Bundle>, RegisterPluginError>
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        let this = Ptr::new(self);

        paths
            .into_par_iter()
            .map(move |path| private_loader::register_plugin(this.as_mut(), path))
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn unregister_plugin(
        &mut self,
        id: &str,
        version: &Version,
    ) -> Result<(), UnregisterPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == (id, version))
            .ok_or(UnregisterPluginError::NotFound)?;
        private_loader::unregister_plugin(&mut self.plugins, index)
    }

    pub fn unregister_plugin_by_bundle(
        &mut self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == *bundle)
            .ok_or(UnregisterPluginError::NotFound)?;
        private_loader::unregister_plugin(&mut self.plugins, index)
    }

    pub fn par_unregister_plugin_by_bundle(
        &mut self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        let index = self
            .plugins
            .par_iter()
            .position_first(|plugin| *plugin == *bundle)
            .ok_or(UnregisterPluginError::NotFound)?;
        private_loader::unregister_plugin(&mut self.plugins, index)
    }

    pub unsafe fn forced_unregister_plugin(
        &mut self,
        index: usize,
    ) -> Result<(), UnregisterPluginError> {
        private_loader::forced_unregister_plugin(&mut self.plugins, index)
    }

    pub fn unload_plugin(&mut self, id: &str, version: &Version) -> Result<(), UnloadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == (id, version))
            .ok_or(UnloadPluginError::NotFound)?;
        private_loader::unload_plugin(&mut self.plugins, index)
    }

    pub fn par_unload_plugin(
        &mut self,
        id: &str,
        version: &Version,
    ) -> Result<(), UnloadPluginError> {
        let index = self
            .plugins
            .par_iter()
            .position_first(|plugin| *plugin == (id, version))
            .ok_or(UnloadPluginError::NotFound)?;
        private_loader::unload_plugin(&mut self.plugins, index)
    }

    pub fn unload_plugin_by_bundle(&mut self, bundle: &Bundle) -> Result<(), UnloadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == *bundle)
            .ok_or(UnloadPluginError::NotFound)?;
        private_loader::unload_plugin(&mut self.plugins, index)
    }

    pub fn par_unload_plugin_by_bundle(
        &mut self,
        bundle: &Bundle,
    ) -> Result<(), UnloadPluginError> {
        let index = self
            .plugins
            .par_iter()
            .position_first(|plugin| *plugin == *bundle)
            .ok_or(UnloadPluginError::NotFound)?;
        private_loader::unload_plugin(&mut self.plugins, index)
    }

    pub unsafe fn forced_unload_plugin(&mut self, index: usize) -> Result<(), UnloadPluginError> {
        private_loader::forced_unload_plugin(&mut self.plugins, index)
    }

    pub fn get_plugin(&self, id: &str, version: &Version) -> Option<&Plugin<'a, O, I>> {
        self.plugins.iter().find(|plugin| **plugin == (id, version))
    }

    pub fn par_get_plugin(&self, id: &str, version: &Version) -> Option<&Plugin<'a, O, I>> {
        self.plugins
            .par_iter()
            .find_first(|plugin| **plugin == (id, version))
    }

    pub fn get_plugin_by_bundle(&self, bundle: &Bundle) -> Option<&Plugin<'a, O, I>> {
        self.plugins.iter().find(|plugin| *plugin == bundle)
    }

    pub fn par_get_plugin_by_bundle(&self, bundle: &Bundle) -> Option<&Plugin<'a, O, I>> {
        self.plugins
            .par_iter()
            .find_first(|plugin| *plugin == bundle)
    }

    pub fn get_plugin_mut(&mut self, id: &str, version: &Version) -> Option<&mut Plugin<'a, O, I>> {
        self.plugins
            .iter_mut()
            .find(|plugin| **plugin == (id, version))
    }

    pub fn par_get_plugin_mut(
        &mut self,
        id: &str,
        version: &Version,
    ) -> Option<&mut Plugin<'a, O, I>> {
        self.plugins
            .par_iter_mut()
            .find_first(|plugin| **plugin == (id, version))
    }

    pub fn get_plugin_mut_by_bundle(&mut self, bundle: &Bundle) -> Option<&mut Plugin<'a, O, I>> {
        self.plugins.iter_mut().find(|plugin| *plugin == bundle)
    }

    pub fn par_get_plugin_mut_by_bundle(
        &mut self,
        bundle: &Bundle,
    ) -> Option<&mut Plugin<'a, O, I>> {
        self.plugins
            .par_iter_mut()
            .find_first(|plugin| *plugin == bundle)
    }

    pub fn get_plugins_by_id(&self, id: &str) -> Vec<&Plugin<'a, O, I>> {
        self.plugins
            .iter()
            .filter(|plugin| plugin.info.bundle.id == id)
            .collect()
    }

    pub fn par_get_plugins_by_id(&self, id: &str) -> Vec<&Plugin<'a, O, I>> {
        self.plugins
            .par_iter()
            .filter(|plugin| plugin.info.bundle.id == id)
            .collect()
    }

    pub fn get_plugins_by_id_mut(&mut self, id: &str) -> Vec<&mut Plugin<'a, O, I>> {
        self.plugins
            .iter_mut()
            .filter(|plugin| plugin.info.bundle.id == id)
            .collect()
    }

    pub fn par_get_plugins_by_id_mut(&mut self, id: &str) -> Vec<&mut Plugin<'a, O, I>> {
        self.plugins
            .par_iter_mut()
            .filter(|plugin| plugin.info.bundle.id == id)
            .collect()
    }

    //TODO: Добавить функции для отслеживания загрузки и выгрузки
    //		менеджеров или плагинов

    pub const fn get_plugins(&self) -> &Vec<Plugin<'a, O, I>> {
        &self.plugins
    }

    pub const fn get_registry(&self) -> &Registry<O> {
        &self.registry
    }

    pub const fn get_requests(&self) -> &Requests {
        &self.requests
    }

    pub fn call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<O>, PluginCallRequestError> {
        self.plugins
            .iter()
            .filter_map(|plugin| {
                let check_version = self.plugins.iter().find(|pl| {
                    pl.info.bundle.id == plugin.info.bundle.id
                        && pl.info.bundle.version > plugin.info.bundle.version
                });

                match check_version {
                    Some(_) => None,
                    None => Some(plugin.call_request(name, args)),
                }
            })
            .collect()
    }

    pub fn par_call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<O>, PluginCallRequestError> {
        let requests: Vec<_> = self
            .plugins
            .iter()
            .filter_map(|plugin| {
                let check_version = self.plugins.iter().find(|pl| {
                    pl.info.bundle.id == plugin.info.bundle.id
                        && pl.info.bundle.version > plugin.info.bundle.version
                });

                match check_version {
                    Some(_) => None,
                    None => Some(&plugin.requests),
                }
            })
            .collect();

        requests
            .into_par_iter()
            .map(|requests| {
                requests
                    .par_iter()
                    .find_map_first(|request| match request.name() == name {
                        true => Some(request.call(args)),
                        false => None,
                    })
                    .ok_or(PluginCallRequestError::NotFound)
            })
            .collect()
    }
}

impl<O: Send + Sync + 'static, I: Info + 'static> Loader<'static, O, I> {
    pub fn load_plugin(&mut self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == (id, version))
            .ok_or(LoadPluginError::NotFound)?;
        private_loader::load_plugin(self, index)
    }

    pub fn par_load_plugin(&mut self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        let index = self
            .plugins
            .par_iter()
            .position_first(|plugin| *plugin == (id, version))
            .ok_or(LoadPluginError::NotFound)?;
        private_loader::load_plugin(self, index)
    }

    pub fn load_plugin_by_bundle(&mut self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == *bundle)
            .ok_or(LoadPluginError::NotFound)?;
        private_loader::load_plugin(self, index)
    }

    pub fn par_load_plugin_by_bundle(&mut self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        let index = self
            .plugins
            .par_iter()
            .position_first(|plugin| *plugin == *bundle)
            .ok_or(LoadPluginError::NotFound)?;
        private_loader::load_plugin(self, index)
    }

    pub unsafe fn forced_load_plugin(
        &mut self,
        index: usize,
        depends: Vec<(Bundle, bool)>,
    ) -> Result<(), LoadPluginError> {
        private_loader::forced_load_plugin(self, index, depends)
    }

    pub fn load_plugin_now(
        &mut self,
        path: &str,
    ) -> Result<Bundle, (Option<RegisterPluginError>, Option<LoadPluginError>)> {
        let bundle = private_loader::register_plugin(self, path).map_err(|e| (Some(e), None))?;
        self.load_plugin_by_bundle(&bundle)
            .map_err(|e| (None, Some(e)))?;
        Ok(bundle)
    }

    pub fn load_plugins<'b, P>(
        &mut self,
        paths: P,
    ) -> Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoIterator<Item = &'b str>,
    {
        let bundles = self.register_plugins(paths).map_err(|e| (Some(e), None))?;

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let result: Vec<_> = self
            .plugins
            .iter()
            .enumerate()
            .filter_map(|(index, plugin)| {
                let find_plugin = self.plugins.iter().find(|pl| {
                    pl.info
                        .info
                        .depends()
                        .iter()
                        .chain(pl.info.info.optional_depends().iter())
                        .any(|d| {
                            *d == plugin.info.bundle
                                && self
                                    .plugins
                                    .iter()
                                    .find(|p| {
                                        d.version.matches(&p.info.bundle.version)
                                            && p.info.bundle.version > plugin.info.bundle.version
                                    })
                                    .is_none()
                        })
                });

                match find_plugin {
                    Some(_) => None,
                    None => Some(index),
                }
            })
            .collect();

        result.into_iter().try_for_each(|index| {
            private_loader::load_plugin(self, index).map_err(|e| (None, Some(e)))
        })?;

        Ok(bundles)
    }

    pub fn par_load_plugins<'b, P>(
        &mut self,
        paths: P,
    ) -> Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        let bundles = self
            .par_register_plugins(paths)
            .map_err(|e| (Some(e), None))?;

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let result: Vec<_> = self
            .plugins
            .par_iter()
            .enumerate()
            .filter_map(|(index, plugin)| {
                let find_plugin = self.plugins.iter().find(|pl| {
                    pl.info
                        .info
                        .depends()
                        .iter()
                        .chain(pl.info.info.optional_depends().iter())
                        .any(|d| {
                            *d == plugin.info.bundle
                                && self
                                    .plugins
                                    .iter()
                                    .find(|p| {
                                        d.version.matches(&p.info.bundle.version)
                                            && p.info.bundle.version > plugin.info.bundle.version
                                    })
                                    .is_none()
                        })
                });

                match find_plugin {
                    Some(_) => None,
                    None => Some(index),
                }
            })
            .collect();

        let this = Ptr::new(self);
        result.into_par_iter().try_for_each(move |index| {
            private_loader::load_plugin(this.as_mut(), index).map_err(|e| (None, Some(e)))
        })?;

        Ok(bundles)
    }

    pub fn load_only_used_plugins<'b, P>(
        &mut self,
        paths: P,
    ) -> Result<
        Vec<Bundle>,
        (
            Option<RegisterPluginError>,
            Option<UnregisterPluginError>,
            Option<LoadPluginError>,
        ),
    >
    where
        P: IntoIterator<Item = &'b str>,
    {
        let mut bundles = self
            .register_plugins(paths)
            .map_err(|e| (Some(e), None, None))?;

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let (used, unused): (Vec<_>, Vec<_>) = self
            .plugins
            .iter()
            .enumerate()
            .filter_map(|(index, plugin)| {
                let find_plugin = self.plugins.iter().find(|pl| {
                    pl.info
                        .info
                        .depends()
                        .iter()
                        .chain(pl.info.info.optional_depends().iter())
                        .any(|d| {
                            *d == plugin.info.bundle
                                && self
                                    .plugins
                                    .iter()
                                    .find(|p| {
                                        d.version.matches(&p.info.bundle.version)
                                            && p.info.bundle.version > plugin.info.bundle.version
                                    })
                                    .is_none()
                        })
                });

                match find_plugin {
                    Some(_) => None,
                    None => Some(index),
                }
            })
            .partition(|index| {
                let bundle = &self.plugins[*index].info.bundle;

                // Ищем самую высокую версию
                self.plugins
                    .iter()
                    .find(|pl| {
                        pl.info.bundle.id == bundle.id && pl.info.bundle.version > bundle.version
                    })
                    .is_none()
            });

        used.into_iter().try_for_each(|index| {
            private_loader::load_plugin(self, index).map_err(|e| (None, None, Some(e)))
        })?;

        let mut old_indexs = vec![];
        let mut unused = unused.into_iter();

        while let Some(index) = unused.next() {
            let swap = old_indexs
                .iter()
                .fold(0, |acc, i| if index > *i { acc + 1 } else { acc });

            let new_index = index - swap;

            let bundle = &self.plugins[new_index].info.bundle;
            bundles.retain(|b| *b != *bundle);

            private_loader::unregister_plugin(&mut self.plugins, new_index)
                .map_err(|e| (None, Some(e), None))?;

            old_indexs.push(index);
        }

        Ok(bundles)
    }

    pub fn par_load_only_used_plugins<'b, P>(
        &mut self,
        paths: P,
    ) -> Result<
        Vec<Bundle>,
        (
            Option<RegisterPluginError>,
            Option<UnregisterPluginError>,
            Option<LoadPluginError>,
        ),
    >
    where
        P: IntoParallelIterator<Item = &'b str>,
    {
        let bundles = self
            .par_register_plugins(paths)
            .map_err(|e| (Some(e), None, None))?;

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let (used, unused): (Vec<_>, Vec<_>) = self
            .plugins
            .iter()
            .enumerate()
            .filter_map(|(index, plugin)| {
                let find_plugin = self.plugins.iter().find(|pl| {
                    pl.info
                        .info
                        .depends()
                        .iter()
                        .chain(pl.info.info.optional_depends().iter())
                        .any(|d| {
                            *d == plugin.info.bundle
                                && self
                                    .plugins
                                    .iter()
                                    .find(|p| {
                                        d.version.matches(&p.info.bundle.version)
                                            && p.info.bundle.version > plugin.info.bundle.version
                                    })
                                    .is_none()
                        })
                });

                match find_plugin {
                    Some(_) => None,
                    None => Some(index),
                }
            })
            .partition(|index| {
                let bundle = &self.plugins[*index].info.bundle;

                // Ищем самую высокую версию
                self.plugins
                    .iter()
                    .find(|pl| {
                        pl.info.bundle.id == bundle.id && pl.info.bundle.version > bundle.version
                    })
                    .is_none()
            });

        let this = Ptr::new(self);
        used.into_iter().try_for_each(|index| {
            private_loader::load_plugin(this.as_mut(), index).map_err(|e| (None, None, Some(e)))
        })?;

        let mut old_indexs = vec![];
        let mut unused = unused.into_iter();

        while let Some(index) = unused.next() {
            let swap = old_indexs
                .iter()
                .fold(0, |acc, i| if index > *i { acc + 1 } else { acc });

            private_loader::unregister_plugin(&mut this.as_mut().plugins, index - swap)
                .map_err(|e| (None, Some(e), None))?;

            old_indexs.push(index);
        }

        Ok(bundles)
    }
}

impl<O: Send + Sync, I: Info> Drop for Loader<'_, O, I> {
    fn drop(&mut self) {
        self.stop().unwrap();
    }
}

mod private_loader {
    use std::path::Path;

    use crate::{
        utils::{
            LoadPluginError, Ptr, RegisterManagerError, RegisterPluginError, StopLoaderError,
            UnloadPluginError, UnregisterManagerError, UnregisterPluginError,
        },
        Api, Bundle, Depend, Info, LoadPluginContext, Manager, Plugin, PluginInfo,
        RegisterPluginContext,
    };

    pub fn stop_plugins<O: Send + Sync, I: Info>(
        loader: &mut super::Loader<'_, O, I>,
    ) -> Result<(), StopLoaderError> {
        // Сортируем плагины в порядке их зависимостей
        let sort_plugins = sort_plugins(
            &loader.plugins,
            loader
                .plugins
                .iter()
                .enumerate()
                .map(|(index, _)| index)
                .collect(),
        );

        // Выгружаем плагины
        let errors = sort_plugins
            .iter()
            .map(|index| {
                forced_unload_plugin(&mut loader.plugins, index.clone())
                    .map_err(|e| UnregisterPluginError::UnloadError(e))
            })
            .partition::<Vec<_>, _>(|r| r.is_err())
            .0;

        if !errors.is_empty() {
            return Err(StopLoaderError::UnregisterPluginFailed(
                errors.into_iter().map(|r| r.err().unwrap()).collect(),
            ));
        }

        //TODO: Добавить debug вывод
        let errors = (0..loader.plugins.len())
            .map(|_| forced_unregister_plugin(&mut loader.plugins, 0_usize))
            .partition::<Vec<_>, _>(|r| r.is_err())
            .0;

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterPluginFailed(
                errors.into_iter().map(|r| r.err().unwrap()).collect(),
            )),
            false => Ok(()),
        }
    }

    pub fn stop_managers<'a, O: Send + Sync, I: Info>(
        loader: &'a mut super::Loader<'_, O, I>,
    ) -> Result<(), StopLoaderError> {
        // Открепляем менеджеры плагинов от загрузчика
        let mut errors = vec![];
        while !loader.managers.is_empty() {
            if let Err(e) = forced_unregister_manager(&mut loader.managers, 0_usize) {
                errors.push(e);
            }
        }

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterManagerFailed(errors)),
            false => Ok(()),
        }
    }

    /*
        TODO: Изменить сортировку плагинов.
        В аргументы функции необходимо передавать список всех плагинов
        и необязательный набор индексов плагинов для точечной сортировки.
        На выходе должен выдаваться индекс начала сортированных плагинов.

        Механика сортировки заключается в смещение в конец списка плагинов
        попутно сортируя их.
    */
    // pub fn sort_plugins<'a, O: Send + Sync, I: Info>(
    //     plugins: &mut Vec<Plugin<'a, O, I>>,
    //     plugins_set: Option<Vec<usize>>,
    // ) -> usize

    // Продвинутая древовидная сортировка
    pub fn sort_plugins<'a, O: Send + Sync, I: Info>(
        plugins: &Vec<Plugin<'a, O, I>>,
        plugins_set: Vec<usize>,
    ) -> Vec<usize> {
        let mut result = vec![];

        'outer: for index in plugins_set.iter() {
            let bundle = &plugins[*index].info.bundle;

            let find_plugin = plugins.iter().enumerate().find_map(|(i, pl)| {
                pl.info
                    .info
                    .depends()
                    .iter()
                    .chain(pl.info.info.optional_depends().iter())
                    .any(|d| {
                        *d == *bundle
                            && plugins
                                .iter()
                                .find(|p| {
                                    d.version.matches(&p.info.bundle.version)
                                        && p.info.bundle.version > bundle.version
                                })
                                .is_none()
                    })
                    .then_some(i)
            });

            if find_plugin.is_some()
                && plugins_set
                    .iter()
                    .find(|i| **i == find_plugin.unwrap())
                    .is_some()
            {
                continue 'outer;
            }

            sort_pick(plugins, &plugins_set, index, &mut result);
        }

        result
    }

    pub fn sort_pick<'a, O: Send + Sync, I: Info>(
        plugins: &Vec<Plugin<'a, O, I>>,
        plugins_set: &Vec<usize>,
        index: &usize,
        result: &mut Vec<usize>,
    ) {
        result.push(index.clone());

        let plugin_info = &plugins[*index].info;
        let depends = plugin_info
            .info
            .depends()
            .iter()
            .chain(plugin_info.info.optional_depends().iter());
        'outer: for depend in depends {
            if !result.iter().any(|inx| {
                *depend == plugins[*inx].info.bundle
                    && plugins
                        .iter()
                        .find(|p| {
                            depend.version.matches(&p.info.bundle.version)
                                && p.info.bundle.version > plugins[*inx].info.bundle.version
                        })
                        .is_none()
            }) {
                let mut plugin = None;

                for index in plugins_set.iter() {
                    let plug_info = &plugins[*index].info;
                    if *depend == plug_info.bundle
                        && plugins
                            .iter()
                            .find(|p| {
                                depend.version.matches(&p.info.bundle.version)
                                    && p.info.bundle.version > plug_info.bundle.version
                            })
                            .is_none()
                    {
                        plugin = Some(index);
                        continue;
                    }

                    if !result
                        .iter()
                        .any(|inx| plugins[*inx].info.bundle == plug_info.bundle)
                        && (plug_info.info.depends().contains(depend)
                            || plug_info.info.optional_depends().contains(depend))
                    {
                        continue 'outer;
                    }
                }

                if let Some(index) = plugin {
                    sort_pick(plugins, plugins_set, index, result);
                }
            }
        }
    }

    pub fn forced_register_manager<'a, O: Send + Sync, I: Info>(
        loader: &mut super::Loader<'a, O, I>,
        mut manager: Box<dyn Manager<'a, O, I>>,
    ) -> Result<(), RegisterManagerError> {
        manager.as_mut().register_manager()?;
        loader.managers.push(manager);
        Ok(())
    }

    pub fn register_manager<'a, O: Send + Sync, I: Info>(
        loader: &mut super::Loader<'a, O, I>,
        manager: Box<dyn Manager<'a, O, I>>,
    ) -> Result<(), RegisterManagerError> {
        if let Some(_) = loader.managers.iter().find(|m| manager == **m) {
            return Err(RegisterManagerError::AlreadyOccupiedFormat(
                manager.format().to_string(),
            ));
        }

        forced_register_manager(loader, manager)
    }

    pub fn forced_unregister_manager<O: Send + Sync, I: Info>(
        managers: &mut Vec<Box<dyn Manager<'_, O, I>>>,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        match managers.remove(index).unregister_manager() {
            Ok(_) => Ok(()),
            Err(e) => Err(UnregisterManagerError::UnregisterManagerByManager(e)),
        }
    }

    pub fn unregister_manager<O: Send + Sync, I: Info>(
        loader: &mut super::Loader<'_, O, I>,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        let manager = &loader.managers[index];

        // Получаем все плагины, относящиеся к менеджеру
        let plugins_from_manager = loader
            .plugins
            .iter()
            .enumerate()
            .filter_map(
                |(index, plugin)| match *plugin.manager.as_ref() == *manager {
                    true => Some(index),
                    false => None,
                },
            )
            .collect();

        // Сортируем плагины менеджера в порядке их зависимостей
        let sort_plugins = sort_plugins(&loader.plugins, plugins_from_manager);

        // Выгружаем плагины
        for index in sort_plugins.iter() {
            unload_plugin(&mut loader.plugins, index.clone()).map_err(|e| {
                UnregisterManagerError::UnregisterPlugin(UnregisterPluginError::UnloadError(e))
            })?;
        }

        let mut old_indexs = vec![];
        let mut sort_plugins = sort_plugins.into_iter();

        while let Some(index) = sort_plugins.next() {
            let swap = old_indexs
                .iter()
                .fold(0, |acc, i| if index > *i { acc + 1 } else { acc });

            forced_unregister_plugin(&mut loader.plugins, index - swap)
                .map_err(|e| UnregisterManagerError::UnregisterPlugin(e))?;

            old_indexs.push(index);
        }

        // Выгружаем менеджер
        forced_unregister_manager(&mut loader.managers, index)
    }

    pub fn forced_register_plugin<'a, O: Send + Sync, I: Info>(
        plugins: &mut Vec<Plugin<'a, O, I>>,
        manager: Ptr<'a, Box<dyn Manager<'a, O, I>>>,
        plugin_info: PluginInfo<I>,
    ) -> Result<Bundle, RegisterPluginError> {
        let bundle = plugin_info.bundle.clone();
        plugins.push(Plugin::<'a>::new(manager, plugin_info));
        Ok(bundle)
    }

    pub fn register_plugin<'a, O: Send + Sync, I: Info>(
        loader: &mut super::Loader<'a, O, I>,
        path: &str,
    ) -> Result<Bundle, RegisterPluginError> {
        let path = Path::new(path).to_path_buf();

        if !path.is_dir() {
            return Err(RegisterPluginError::NotFound);
        }

        if let None = path.extension() {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }

        let bundle = Bundle::from_filename(path.file_name().unwrap())?;

        // Проверяем, есть ли уже такой плагин
        if loader.get_plugin_by_bundle(&bundle).is_some() {
            return Err(RegisterPluginError::AlreadyExistsIDAndVersion(
                bundle.id.clone(),
                bundle.version.clone(),
            ));
        }

        // Ищем подходящий менеджер
        let plugin_format = bundle.format.clone();
        let manager = loader
            .get_manager_mut(plugin_format.as_str())
            .ok_or(RegisterPluginError::UnknownManagerFormat(plugin_format))?;

        // Менеджер регистрирует плагин
        let info = manager.register_plugin(RegisterPluginContext {
            path: &path,
            bundle: &bundle,
        })?;
        let plugin_info = PluginInfo { path, bundle, info };

        // Регистрируем плагин
        let manager = Ptr::<'a>::new(manager);
        forced_register_plugin(&mut loader.plugins, manager, plugin_info)
    }

    pub fn forced_unregister_plugin<O: Send + Sync, I: Info>(
        plugins: &mut Vec<Plugin<'_, O, I>>,
        index: usize,
    ) -> Result<(), UnregisterPluginError> {
        let plugin = plugins.remove(index);
        plugin.manager.as_mut().unregister_plugin(&plugin)?;
        Ok(())
    }

    pub fn unregister_plugin<'a, O: Send + Sync, I: Info>(
        plugins: &mut Vec<Plugin<'_, O, I>>,
        index: usize,
    ) -> Result<(), UnregisterPluginError> {
        unload_plugin(plugins, index)?;
        forced_unregister_plugin(plugins, index)
    }

    pub fn forced_load_plugin<O: Send + Sync, I: Info>(
        loader: *mut super::Loader<'static, O, I>,
        index: usize,
        depends: Vec<(Bundle, bool)>,
    ) -> Result<(), LoadPluginError> {
        let manager = Ptr::new(unsafe { &*loader }.plugins[index].manager.as_ptr());

        // Получаем плагин и его зависимости
        let plugin = &mut unsafe { &mut *loader }.plugins[index];

        // Делим зависимости
        let mut deps = vec![];
        let mut opt_deps = vec![];

        for (bundle, is_depend) in depends {
            match is_depend {
                true => deps.push(bundle),
                false => opt_deps.push(bundle),
            }
        }

        // Загружаем плагин
        let bundle = plugin.info.bundle.clone();

        manager.as_mut().load_plugin(
            LoadPluginContext::new(plugin, &unsafe { &*loader }.requests),
            Api::new(Ptr::new(loader), bundle, deps, opt_deps),
        )?;

        plugin.is_load = true;

        Ok(())
    }

    fn load_depends<'a, O, I, IT>(
        loader: &'a mut super::Loader<'static, O, I>,
        depends_iter: IT,
    ) -> Result<(Vec<(Bundle, bool)>, Vec<Depend>), LoadPluginError>
    where
        O: Send + Sync,
        I: Info,
        IT: IntoIterator<Item = (bool, Depend)>,
    {
        let mut found_depends = vec![];
        let mut not_found_depends = vec![];

        for (is_depend, depend) in depends_iter.into_iter() {
            if let Some((index, plugin)) = loader.plugins.iter().enumerate().find(|(_, plugin)| {
                depend == plugin.info.bundle
                    && loader
                        .plugins
                        .iter()
                        .find(|p| {
                            depend.version.matches(&p.info.bundle.version)
                                && p.info.bundle.version > plugin.info.bundle.version
                        })
                        .is_none()
            }) {
                found_depends.push((plugin.info.bundle.clone(), is_depend));
                load_plugin(loader, index).map_err(|e| LoadPluginError::LoadDependency {
                    depend: depend,
                    error: Box::new(e),
                })?;
            } else if is_depend {
                not_found_depends.push(depend);
            }
        }
        Ok((found_depends, not_found_depends))
    }

    fn check_requests<O: Send + Sync, I: Info>(
        loader: &mut super::Loader<'static, O, I>,
        index: usize,
    ) -> Vec<String> {
        let mut plugin_requests = loader.plugins[index].requests.iter();
        loader
            .requests
            .iter()
            .filter_map(|req| match plugin_requests.any(|r| r.name() == req.name) {
                true => None,
                false => Some(req.name.clone()),
            })
            .collect()
    }

    pub fn load_plugin<O: Send + Sync, I: Info>(
        loader: &mut super::Loader<'static, O, I>,
        index: usize,
    ) -> Result<(), LoadPluginError> {
        if loader.plugins[index].is_load {
            return Ok(());
        }

        // Загружаем зависимости
        let info = &loader.plugins[index].info;
        let depends_iter = info
            .info
            .depends()
            .clone()
            .into_iter()
            .map(|d| (true, d))
            .chain(
                info.info
                    .optional_depends()
                    .clone()
                    .into_iter()
                    .map(|d| (false, d)),
            );
        let (found_depends, not_found_depends) = load_depends(loader, depends_iter)?;

        if !not_found_depends.is_empty() {
            return Err(LoadPluginError::NotFoundDependencies(not_found_depends));
        }

        // Загружаем плагин
        forced_load_plugin(loader, index, found_depends)?;

        // Проверяем наличие запрашиваемых функций
        let not_found_requests = check_requests(loader, index);

        if !not_found_requests.is_empty() {
            loader.plugins[index].is_load = false;
            return Err(LoadPluginError::RequestsNotFound(not_found_requests));
        }

        Ok(())
    }

    pub fn forced_unload_plugin<O: Send + Sync, I: Info>(
        plugins: &mut Vec<Plugin<'_, O, I>>,
        index: usize,
    ) -> Result<(), UnloadPluginError> {
        if plugins[index].is_load {
            plugins[index]
                .manager
                .as_mut()
                .unload_plugin(&plugins[index])?;
        }

        plugins[index].is_load = false;

        Ok(())
    }

    pub fn unload_plugin<'a, O: Send + Sync, I: Info>(
        plugins: &mut Vec<Plugin<'_, O, I>>,
        index: usize,
    ) -> Result<(), UnloadPluginError> {
        if plugins[index].is_load {
            let bundle = &plugins[index].info.bundle;

            // Проверяем, что плагин не используется как зависимость загруженными плагинами
            plugins.iter().try_for_each(|plug| {
                let plug_info = &plug.info;

                let find_depend = plug_info
                    .info
                    .depends()
                    .iter()
                    .chain(plug_info.info.optional_depends().iter())
                    .find(|depend| {
                        **depend == *bundle
                            && plugins
                                .iter()
                                .find(|p| {
                                    depend.version.matches(&p.info.bundle.version)
                                        && p.info.bundle.version > bundle.version
                                })
                                .is_none()
                    })
                    .is_some();
                match plug.is_load && find_depend {
                    true => Err(UnloadPluginError::CurrentlyUsesDepend {
                        plugin: plug_info.bundle.clone(),
                        depend: bundle.clone(),
                    }),
                    false => Ok(()),
                }
            })?;
        }

        forced_unload_plugin(plugins, index)
    }
}
