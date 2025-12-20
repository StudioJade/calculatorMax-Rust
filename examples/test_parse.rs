use calculator_max::calculator::mods::{SimplifiedMod, Mod};

fn main() {
    let content = r#"[a.b.c]
name = "test_func"
type = "fun"
needs = ["x", "y"]
method = "x * y + 10"
"#;

    // Remove the first line (table header) and parse the rest as SimplifiedMod
    let content_without_header = content.lines().skip(1).collect::<Vec<_>>().join("\n");
    println!("Content to parse: {}", content_without_header);
    
    match toml::from_str::<SimplifiedMod>(&content_without_header) {
        Ok(simplified_mod) => {
            println!("Successfully parsed:");
            println!("  Name: {:?}", simplified_mod.name);
            println!("  Type: {:?}", simplified_mod.mod_type);
            println!("  Needs: {:?}", simplified_mod.needs);
            println!("  Method: {:?}", simplified_mod.method);
            println!("  Res: {:?}", simplified_mod.res);
        }
        Err(e) => {
            eprintln!("Failed to parse: {}", e);
        }
    }
}