use eframe::egui;
use plux_mock::MockManager;
use plux_rs::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod plugins;
use plugins::gui::{self, help_v1, test_v1, user_v1};
use plugins::utils::get_plugin_path;

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
    loader: SimpleLoader,
    // Loaded plugin bundles
    plugin_bundles: Vec<Bundle>,
    // Currently selected plugin
    selected_plugin: Option<Bundle>,
    // Current user ID
    current_user_id: Arc<Mutex<i32>>,
    // Plugin output
    plugin_output: String,
    // Input field state
    input_text: String,
}

impl GuiApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Try to create and configure the plugin loader
        let mut loader = SimpleLoader::new();

        // Create mock plugins in memory
        let mut plugins = HashMap::new();

        // Define plugins
        gui::help_v1::insert_plugin(&mut plugins);
        gui::user_v1::insert_plugin(&mut plugins);
        gui::test_v1::insert_plugin(&mut plugins);

        let current_user_id = Arc::new(Mutex::new(1)); // Default user

        loader.context(|mut ctx| {
            // Register the Mock plugin manager with our in-memory plugins
            ctx.register_manager(MockManager::from_plugins(plugins))?;

            // Register functions that will be available to plugins
            ctx.register_function(get_user_data(current_user_id.clone()));

            // Define requests that plugins must implement
            ctx.register_request(Request::new(
                "render".to_string(),
                vec![],
                Some(VariableType::String),
            ));

            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })?;

        // Try to load GUI plugins
        let paths = [help_v1::FILENAME, user_v1::FILENAME, test_v1::FILENAME]
            .into_iter()
            .map(|filename| get_plugin_path(format!("gui/{}", filename)))
            .collect::<Vec<_>>();

        let plugin_bundles = loader
            .load_plugins(paths.iter().map(|path| path.to_str().unwrap()))
            .unwrap();

        Ok(Self {
            loader,
            plugin_bundles,
            selected_plugin: None,
            current_user_id,
            plugin_output: "GUI Application with Plugin Support\n\nThis is a demonstration of a GUI application with plugin support using Plux.\nSelect plugins from the left panel to see their output.".to_string(),
            input_text: String::new(),
        })
    }

    fn render_plugin_ui(&mut self, ui: &mut egui::Ui, plugin_page: String) {
        // Parse the JSON output from the plugin and render UI components
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(plugin_page.as_str()) {
            if let serde_json::Value::Object(components) = json_value {
                for (_component_id, component_data) in components {
                    if let serde_json::Value::Object(data) = component_data {
                        // Get component type from the "type" field
                        if let Some(component_type) = data.get("type").and_then(|v| v.as_str()) {
                            match component_type {
                                "label" => {
                                    if let Some(text) = data.get("text").and_then(|v| v.as_str()) {
                                        ui.label(text);
                                    }
                                }
                                "button" => {
                                    if let Some(text) = data.get("text").and_then(|v| v.as_str()) {
                                        if ui.button(text).clicked() {
                                            // Handle button click if needed
                                        }
                                    }
                                }
                                "input" => {
                                    if let Some(placeholder) =
                                        data.get("placeholder").and_then(|v| v.as_str())
                                    {
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.input_text)
                                                .hint_text(placeholder),
                                        );
                                    }
                                }
                                _ => {
                                    ui.label(format!("Unknown component type: {}", component_type));
                                }
                            }
                        } else {
                            ui.label("Component missing 'type' field");
                        }
                    }
                }
            }
        } else {
            // If JSON parsing fails, display the raw output
            ui.label(&self.plugin_output);
        }
    }

    fn load_plugin_content(&mut self, plugin_bundle: &Bundle) {
        if self.plugin_bundles.is_empty() {
            self.plugin_output = "No plugins loaded. Check error message above.".to_string();
            return;
        }

        let plugin = match self.loader.get_plugin_by_bundle(plugin_bundle) {
            Some(p) => p,
            None => {
                self.plugin_output = "Error accessing plugin".to_string();
                return;
            }
        };

        match plugin.call_request("render", &[]) {
            Ok(Ok(Some(Variable::String(content)))) => {
                self.plugin_output = content;
                self.selected_plugin = Some(plugin_bundle.clone());
                return;
            }
            Ok(Ok(Some(_))) | Ok(Ok(None)) => {
                self.plugin_output = format!("Plugin '{}' returned no content", plugin_bundle.id);
                return;
            }
            Ok(Err(e)) => {
                self.plugin_output = format!("Error in plugin '{}': {:?}", plugin_bundle.id, e);
                return;
            }
            Err(e) => {
                self.plugin_output = format!("Error in plugin '{}': {:?}", plugin_bundle.id, e);
                return;
            }
        }
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
            for bundle in &self.plugin_bundles.clone() {
                let plugin_name = {
                    let id = bundle.id.as_str();
                    id.chars()
                        .next()
                        .map(|c| c.to_uppercase().collect::<String>() + &id[1..])
                        .unwrap_or_default()
                };
                if ui.button(plugin_name).clicked() {
                    self.load_plugin_content(bundle);
                }
            }

            ui.separator();

            ui.heading("User Selection");

            // User selection
            egui::ComboBox::from_label("Select User")
                .selected_text(format!("User #{}", self.current_user_id.lock().unwrap()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut *self.current_user_id.lock().unwrap(),
                        1,
                        "John Doe (#1)",
                    );
                    ui.selectable_value(
                        &mut *self.current_user_id.lock().unwrap(),
                        2,
                        "Jane Smith (#2)",
                    );
                });

            // Reload current plugin when user changes
            if ui.button("Refresh Plugin").clicked() {
                if let Some(plugin_bundle) = self.selected_plugin.clone() {
                    self.load_plugin_content(&plugin_bundle);
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
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    self.render_plugin_ui(ui, self.plugin_output.clone());
                });
        });
    }
}

// Function to get user data
// This function will be registered with the plugin system and available to plugins
#[plux_rs::function]
fn get_user_data(current_user_id: &Arc<Mutex<i32>>) -> Variable {
    // In a real application, this would fetch from a database
    match *current_user_id.lock().unwrap() {
        1 => Variable::List(vec!["John Doe".into(), 30.into()]),
        2 => Variable::List(vec!["Jane Smith".into(), 28.into()]),
        _ => Variable::Null,
    }
}
