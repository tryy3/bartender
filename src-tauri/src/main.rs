#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

mod cmd;

#[derive(Debug)]
struct Pump {
  id: String,
  ingredient_id: String,
  gpio: u64
}

#[derive(Debug)]
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
}

#[derive(Debug, Serialize)]
struct Response<'a> {
  value: u64,
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

fn main() {
  let default_settings = Settings{
    pumps: vec![
      Pump{
        id: "1".to_string(),
        ingredient_id: "".to_string(),
        gpio: 0,
      },
      Pump{
        id: "2".to_string(),
        ingredient_id: "".to_string(),
        gpio: 1,
      },
      Pump{
        id: "3".to_string(),
        ingredient_id: "".to_string(),
        gpio: 2,
      },
    ]
  };

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
                  let response = Response {
                    value: 5,
                    message: "async response!",
                  };
                  Ok(response)
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}