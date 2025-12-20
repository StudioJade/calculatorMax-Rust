use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimplifiedMod {
    pub name: Option<String>,
    #[serde(default)]
    pub needs: Option<Vec<String>>,
    pub method: Option<String>,
    pub res: Option<f64>,
    #[serde(rename = "type")]
    pub mod_type: Option<String>,
}

fn main() {
    let content = r#"a.b.c = {name = "test_func", type = "fun", needs = ["x", "y"], method = "x * y + 10"}
b.c.d = {name = "test_num", type = "num", res = 1.0}"#;

    match toml::from_str::<HashMap<String, SimplifiedMod>>(content) {
        Ok(simplified_mods) => {
            println!("Successfully parsed:");
            for (mod_id, simplified_mod) in simplified_mods {
                println!("Mod ID: {}", mod_id);
                println!("  Name: {:?}", simplified_mod.name);
                println!("  Type: {:?}", simplified_mod.mod_type);
                println!("  Needs: {:?}", simplified_mod.needs);
                println!("  Method: {:?}", simplified_mod.method);
                println!("  Res: {:?}", simplified_mod.res);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse: {}", e);
        }
    }
}