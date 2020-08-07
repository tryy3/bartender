#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod cmd;

#[derive(Debug, Deserialize, Serialize)]
struct Pump {
    id: String,
    ingredient_id: String,
    gpio: u64
}

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    pumps: Vec<Pump>
}

#[derive(Debug, Deserialize)]
struct RecipePump {
    id: String,
    volume: u64,
}
#[derive(Debug, Deserialize)]
struct PourDrinkPayload {
    pumps: Vec<RecipePump>,
}

// The commands definitions
// Deserialized from JS
#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum Cmd {
    PourDrink {
        payload: PourDrinkPayload,
        callback: String,
        error: String,
    },
    LoadSettings {
        callback: String,
        error: String,
    },
    SaveSettings {
        payload: Settings,
        callback: String,
        error: String,
    },
}

#[derive(Debug, Serialize)]
struct SimpleResponse<'a> {
    message: &'a str,
}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct CommandError<'a> {
    message: &'a str,
}

impl<'a> CommandError<'a> {
    fn new(message: &'a str) -> Self {
        Self { message }
    }
}

impl<'a> std::fmt::Display for CommandError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

fn read_config() -> Result<Settings, Box<dyn Error>> {
    let config_path = Path::new("config.json");
    let display = config_path.display();

    if !config_path.exists() {
        let new_config = match File::create(&config_path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let default_settings = json!({
            "pumps": [
                {
                    "id": "1",
                    "ingredient_id": "",
                    "gpio": 0,
                },
                {
                    "id": "2",
                    "ingredient_id": "",
                    "gpio": 1,
                },
                {
                    "id": "3",
                    "ingredient_id": "",
                    "gpio": 2,
                }
            ]
        });

        match serde_json::to_writer_pretty(new_config, &default_settings) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfuly written default config to {}", display),
        }
    }

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(config_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let config = match serde_json::from_reader(reader) {
        Err(why) => panic!("couldn't read file {}: {}", display, why),
        Ok(data) => data,
    };

    Ok(config)
}

fn save_config(settings: Settings) -> Result<(), Box<dyn Error>> {
    let path = Path::new("config.json");
    let display = path.display();

    let config = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match serde_json::to_writer_pretty(config, &settings) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfuly written default config to {}", display),
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        PourDrink { payload, callback, error } => tauri::execute_promise(
                            _webview,
                            move || {
                                println!("{:?}", payload);
                                    let response = SimpleResponse {
                                        message: "async response!",
                                    };
                                    Ok(response)
                            },
                            callback,
                            error,
                        ),
                        LoadSettings { callback, error } => tauri::execute_promise(
                            _webview, 
                            move || {
                                match read_config() {
                                    Err(why) => {
                                        println!("Unable to load config: {}", why);
                                        Err(CommandError::new("Unable to load config").into())
                                    },
                                    Ok(settings) => Ok(settings)
                                }
                            },
                            callback, 
                            error
                        ),
                        SaveSettings { payload, callback, error } => tauri::execute_promise(
                            _webview, 
                            move || {
                                match save_config(payload) {
                                    Err(why) =>{
                                        println!("unable to save config: {}", why);
                                        Err(CommandError::new("Unable to save config").into())
                                    },
                                    Ok(_) => Ok(SimpleResponse{message:"Successfully saved the config"})
                                }
                            },
                            callback, 
                            error
                        )
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
    
    Ok(())
}