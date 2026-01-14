use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use plux_mock::MockManager;
use plux_rs::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod plugins;
use plugins::web_server_plugin::{self, api_v1, status_v1, user_v1};
use plugins::utils::get_plugin_path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server at http://127.0.0.1:8080");

    // Try to create and configure the plugin loader
    let mut loader = SimpleLoader::new();

    // Create mock plugins in memory
    let mut plugins = HashMap::new();

    // Define plugins
    web_server_plugin::api_v1::insert_plugin(&mut plugins);
    web_server_plugin::status_v1::insert_plugin(&mut plugins);
    web_server_plugin::user_v1::insert_plugin(&mut plugins);

    let current_user_id = Arc::new(Mutex::new(1)); // Default user

    loader.context(|mut ctx| {
        // Register the Mock plugin manager with our in-memory plugins
        ctx.register_manager(MockManager::from_plugins(plugins))?;

        // Register functions that will be available to plugins
        ctx.register_function(get_user_data(current_user_id.clone()));

        // Define requests that plugins must implement
        ctx.register_request(Request::new(
            "handle_request".to_string(),
            vec![],
            Some(VariableType::String),
        ));

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    }).expect("Failed to configure plugin loader");

    // Try to load web plugins
    let paths: Vec<std::path::PathBuf> = [api_v1::FILENAME, status_v1::FILENAME, user_v1::FILENAME]
        .into_iter()
        .map(|filename| get_plugin_path(format!("web/{}", filename)))
        .collect();

    let plugin_bundles = loader
        .load_plugins(paths.iter().map(|path: &std::path::PathBuf| path.to_str().unwrap()))
        .expect("Failed to load plugins");

    // Create shared state for the web server
    let app_state = web::Data::new(AppState {
        loader,
        plugin_bundles,
        current_user_id,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(plugin_list)
            .service(execute_plugin)
            .service(user_select)
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

struct AppState {
    loader: SimpleLoader,
    plugin_bundles: Vec<Bundle>,
    current_user_id: Arc<Mutex<i32>>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Plux Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .container { max-width: 800px; margin: 0 auto; }
        .plugin { border: 1px solid #ddd; margin: 10px 0; padding: 15px; border-radius: 5px; }
        .button { background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 3px; cursor: pointer; }
        .button:hover { background: #0056b3; }
        .user-selector { margin: 20px 0; }
        input[type="number"] { padding: 5px; margin: 0 10px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Plux Web Server</h1>
        <p>This is a demonstration of a web server with plugin support using Plux.</p>
        
        <div class="user-selector">
            <h3>Select User:</h3>
            <select id="userSelect" onchange="changeUser()">
                <option value="1">John Doe (#1)</option>
                <option value="2">Jane Smith (#2)</option>
            </select>
        </div>

        <div id="plugins">
            <h2>Available Plugins:</h2>
            <!-- Plugins will be loaded here -->
        </div>

        <div id="output">
            <h2>Plugin Output:</h2>
            <pre id="outputContent">Select a plugin to see its output...</pre>
        </div>
    </div>

    <script>
        async function loadPlugins() {
            try {
                const response = await fetch('/plugins');
                const plugins = await response.json();
                const pluginsDiv = document.getElementById('plugins');
                
                let html = '<h2>Available Plugins:</h2>';
                plugins.forEach(plugin => {
                    const pluginName = plugin.id.charAt(0).toUpperCase() + plugin.id.slice(1);
                    html += `
                        <div class="plugin">
                            <h3>${pluginName}</h3>
                            <p>Plugin ID: ${plugin.id}</p>
                            <button class="button" onclick="executePlugin('${plugin.id}')">Execute Plugin</button>
                        </div>
                    `;
                });
                pluginsDiv.innerHTML = html;
            } catch (error) {
                console.error('Error loading plugins:', error);
            }
        }

        async function executePlugin(pluginId) {
            try {
                const response = await fetch('/execute', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        plugin_id: pluginId,
                        method: 'GET',
                        path: '/' + pluginId,
                        body: ''
                    })
                });
                const result = await response.json();
                document.getElementById('outputContent').textContent = result.output || result.error;
            } catch (error) {
                console.error('Error executing plugin:', error);
                document.getElementById('outputContent').textContent = 'Error executing plugin: ' + error.message;
            }
        }

        async function changeUser() {
            const userId = document.getElementById('userSelect').value;
            try {
                const response = await fetch('/user/' + userId, { method: 'POST' });
                const result = await response.json();
                console.log('User changed:', result);
                // Reload plugins to refresh with new user context
                loadPlugins();
            } catch (error) {
                console.error('Error changing user:', error);
            }
        }

        // Load plugins when page loads
        loadPlugins();
    </script>
</body>
</html>
    "#)
}

#[get("/plugins")]
async fn plugin_list(data: web::Data<AppState>) -> impl Responder {
    let plugins: Vec<serde_json::Value> = data.plugin_bundles.iter().map(|bundle| {
        serde_json::json!({
            "id": bundle.id,
            "version": bundle.version
        })
    }).collect();
    
    HttpResponse::Ok().json(plugins)
}

#[post("/execute")]
async fn execute_plugin(
    req: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> impl Responder {
    let plugin_id = match req.get("plugin_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing plugin_id"})),
    };

    // Find the plugin by ID
    let plugin_bundle = match data.plugin_bundles.iter().find(|b| b.id == plugin_id) {
        Some(bundle) => bundle,
        None => return HttpResponse::NotFound().json(serde_json::json!({"error": "Plugin not found"})),
    };

    let plugin = match data.loader.get_plugin_by_bundle(plugin_bundle) {
        Some(p) => p,
        None => return HttpResponse::InternalServerError().json(serde_json::json!({"error": "Error accessing plugin"})),
    };

    match plugin.call_request("handle_request", &[]) {
        Ok(Ok(Some(Variable::String(content)))) => {
            HttpResponse::Ok().json(serde_json::json!({"output": content}))
        }
        Ok(Ok(Some(_))) | Ok(Ok(None)) => {
            HttpResponse::Ok().json(serde_json::json!({"output": format!("Plugin '{}' returned no content", plugin_id)}))
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Error in plugin '{}': {:?}", plugin_id, e)}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Error in plugin '{}': {:?}", plugin_id, e)}))
        }
    }
}

#[post("/user/{user_id}")]
async fn user_select(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user_id = path.into_inner();
    *data.current_user_id.lock().unwrap() = user_id;
    
    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("User changed to #{}", user_id),
        "user_id": user_id
    }))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
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