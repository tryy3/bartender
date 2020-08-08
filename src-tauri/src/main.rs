#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use serde::{Serialize};
use std::{thread, time};

mod cmd;
mod config;

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

fn main() -> Result<(), Box<dyn Error>> {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        PourDrink { payload, callback, error } => tauri::execute_promise(
                            _webview,
                            move || {
                                thread::sleep(time::Duration::from_millis(5000));
                                println!("{:?}", payload);
                                let response = SimpleResponse {
                                    message: "Finished",
                                };
                                Ok(response)
                            },
                            callback,
                            error,
                        ),
                        LoadSettings { callback, error } => tauri::execute_promise(
                            _webview, 
                            move || {
                                match config::config::read_config() {
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
                                match config::config::save_config(payload) {
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