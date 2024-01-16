use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::data::rods::{Rod, RodData};
use crate::nay;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserValues {
    pub fish_caught: u32,
    pub money: u32,
    pub rod_name: String,
    pub rod_modifier: Option<String>,
    pub has_seen: Vec<String>
}

impl UserValues {
    pub fn get_rod(&self, rod_data: &RodData) -> Rod {

        let rod_base = rod_data.get_base_by_name(self.rod_name.as_str()).unwrap();

        let modifier = self.rod_modifier.as_ref().map(|modifier|
            rod_data.get_modifier_by_name(modifier.as_str()).unwrap());

        Rod {
            base: rod_base,
            modifier,
        }
    }
}

impl Default for UserValues {
    fn default() -> Self {
        Self {
            fish_caught: 0,
            money: 0,
            rod_name: "Stick with String".to_string(),
            rod_modifier: None,
            has_seen: vec![]
        }
    }
}

pub fn get_userfile_path() -> String {
    "./data/udat.json".to_string()
}

pub fn create_userfile() {
    let raw_path = get_userfile_path();
    let path = Path::new(raw_path.as_str());

    if !path.exists() {
        // make the directories
        if let Err(e) = std::fs::create_dir_all(path.parent().unwrap()) {
            nay!("Failed to create userfile directories: {}", e);
            return;
        }
        // create the file
        if let Err(e) = std::fs::File::create(path) {
            nay!("Failed to create userfile: {}", e);
            return;
        }
    }

    let user_values = UserValues::default();

    let serialized = serde_json::to_string(&user_values).unwrap();

    std::fs::write(path, serialized).unwrap();
}

pub fn read_userfile() -> UserValues {
    let raw_path = get_userfile_path();
    let path = Path::new(raw_path.as_str());

    if !path.exists() {
        create_userfile();
    }

    let contents = std::fs::read_to_string(path).unwrap();

    serde_json::from_str(contents.as_str()).unwrap()
}

pub fn update_userfile(user_values: UserValues) {
    let raw_path = get_userfile_path();
    let path = Path::new(raw_path.as_str());

    if !path.exists() {
        create_userfile();
    }

    let serialized = serde_json::to_string(&user_values).unwrap();

    std::fs::write(path, serialized).unwrap();
}
