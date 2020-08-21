pub mod cmd {
    pub use crate::config::config;
    use serde::Deserialize;

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
        GetIngredients {
            callback: String,
            error: String,
        },
        GetRecipes {
            callback: String,
            error: String,
        },
    }
}
