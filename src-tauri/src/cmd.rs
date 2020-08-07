pub mod cmd {
  use serde::{Deserialize};
  pub use crate::config::config;

  #[derive(Debug, Deserialize)]
  struct RecipePump {
      id: String,
      volume: u64,
  }

  #[derive(Debug, Deserialize)]
  pub struct PourDrinkPayload {
      pumps: Vec<RecipePump>,
  }

  // The commands definitions
  // Deserialized from JS
  #[derive(Debug, Deserialize)]
  #[serde(tag = "cmd", rename_all = "camelCase")]
  pub enum Cmd {
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
          payload: config::Settings,
          callback: String,
          error: String,
      },
  }
}