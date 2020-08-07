pub mod config {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use std::error::Error;


    #[derive(Debug, Deserialize, Serialize)]
    struct Pump {
        id: String,
        ingredient_id: String,
        gpio: u64
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Settings {
        pumps: Vec<Pump>
    }


    pub fn read_config() -> Result<Settings, Box<dyn Error>> {
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

    pub fn save_config(settings: Settings) -> Result<(), Box<dyn Error>> {
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
}