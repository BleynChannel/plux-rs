use eframe::egui;
use plux_lua_manager::LuaManager;
use plux_rs::prelude::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    // Run the GUI application
    eframe::run_native(
        "Plux GUI Application",
        options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(cc)?))),
    )
}

struct GuiApp {
    // Plugin loader
    loader: Loader<'static, FunctionOutput, StdInfo>,
    // Loaded plugin bundles
    plugin_bundles: Vec<Bundle>,
    // Currently selected plugin
    selected_plugin: Option<String>,
    // Current user ID
    current_user_id: i32,
    // Plugin output
    plugin_output: String,
}

impl GuiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Try to create and configure the plugin loader
        let mut loader: Loader<FunctionOutput, StdInfo> = Loader::new();

        loader
            .context(|mut ctx| {
                // Register the Lua plugin manager
                ctx.register_manager(LuaManager::new())?;

                // Register functions that will be available to plugins
                ctx.register_function(get_user_data());
                ctx.register_function(get_system_info());

                // Define requests that plugins must implement
                ctx.register_request(Request::new(
                    "render_ui".to_string(),
                    vec![VariableType::I32],    // user_id
                    Some(VariableType::String), // HTML content
                ));

                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            })?;

        // Try to load GUI plugins
        let plugin_bundles = loader.load_plugins(vec![
            "examples/plugins/gui/dashboard-v1.0.0.lua",
            "examples/plugins/gui/user_profile-v1.0.0.lua",
            "examples/plugins/gui/system_info-v1.0.0.lua",
        ]).unwrap();

        Ok(Self {
            loader,
            plugin_bundles,
            selected_plugin: None,
            current_user_id: 1, // Default user
            plugin_output: "GUI Application with Plugin Support\n\nThis is a demonstration of a GUI application with plugin support using Plux.\nSelect plugins from the left panel to see their output.".to_string(),
        })
    }

    fn render_plugin_ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                ui.label(&self.plugin_output);
            });
    }

    fn load_plugin_content(&mut self, plugin_id: &str) {
        if self.plugin_bundles.is_empty() {
            self.plugin_output = "No plugins loaded. Check error message above.".to_string();
            return;
        }

        for bundle in &self.plugin_bundles {
            // Check if this is the plugin we want to load
            if bundle.id == plugin_id {
                let plugin = match self.loader.get_plugin_by_bundle(bundle) {
                    Some(p) => p,
                    None => {
                        self.plugin_output = "Error accessing plugin".to_string();
                        return;
                    }
                };

                match plugin.call_request("render_ui", &[self.current_user_id.into()]) {
                    Ok(Ok(Some(Variable::String(content)))) => {
                        self.plugin_output = content;
                        self.selected_plugin = Some(plugin_id.to_string());
                        return;
                    }
                    Ok(Ok(Some(_))) | Ok(Ok(None)) => {
                        self.plugin_output =
                            format!("Plugin '{}' returned no content", plugin_id);
                        return;
                    }
                    Ok(Err(e)) => {
                        self.plugin_output = format!("Error in plugin '{}': {:?}", plugin_id, e);
                        return;
                    }
                    Err(e) => {
                        self.plugin_output = format!("Error in plugin '{}': {:?}", plugin_id, e);
                        return;
                    }
                }
            }
        }

        self.plugin_output = format!("Plugin '{}' not found", plugin_id);
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Menu bar
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // Show about dialog
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Plugins");

            // List available plugins
            if ui.button("Dashboard").clicked() {
                self.load_plugin_content("dashboard");
            }

            if ui.button("User Profile").clicked() {
                self.load_plugin_content("user_profile");
            }

            if ui.button("System Info").clicked() {
                self.load_plugin_content("system_info");
            }

            ui.separator();

            ui.heading("User Selection");

            // User selection
            egui::ComboBox::from_label("Select User")
                .selected_text(format!("User #{}", self.current_user_id))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.current_user_id, 1, "John Doe (#1)");
                    ui.selectable_value(&mut self.current_user_id, 2, "Jane Smith (#2)");
                });

            // Reload current plugin when user changes
            if ui.button("Refresh Plugin").clicked() {
                if let Some(plugin_id) = &self.selected_plugin {
                    // Clone the plugin name to avoid borrowing issues
                    let plugin_id_clone = plugin_id.clone();
                    self.load_plugin_content(&plugin_id_clone);
                }
            }

            ui.separator();

            ui.heading("System Info");

            ui.label(format!("OS: {}", std::env::consts::OS));
            ui.label(format!("Architecture: {}", std::env::consts::ARCH));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Plugin Output");

            // Render plugin UI
            self.render_plugin_ui(ui);
        });
    }
}

// Function to get user data
// This function will be registered with the plugin system and available to plugins
#[plux_rs::function]
fn get_user_data(_: (), user_id: &i32) -> Variable {
    // In a real application, this would fetch from a database
    match user_id {
        1 => Variable::List(vec!["John Doe".into(), 30.into()]),
        2 => Variable::List(vec!["Jane Smith".into(), 28.into()]),
        _ => Variable::List(vec!["Unknown User".into(), 0.into()]),
    }
}

// Function to get system information
// This function will be registered with the plugin system and available to plugins
#[plux_rs::function]
fn get_system_info(_: ()) -> Variable {
    let mut sys_info = vec![];
    sys_info.push(std::env::consts::OS.into());
    sys_info.push(std::env::consts::ARCH.into());
    sys_info.push(
        gethostname::gethostname()
            .to_string_lossy()
            .to_string()
            .into(),
    );
    Variable::List(sys_info)
}
