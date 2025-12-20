use calculator_max::calculator::mods::ModManager;

fn main() {
    let mut mod_manager = ModManager::new();
    if let Err(e) = mod_manager.load_mods() {
        eprintln!("Failed to load mods: {}", e);
    } else {
        println!("Successfully loaded mods");
        let mod_list = mod_manager.list_mods();
        println!("Available mods: {:?}", mod_list);
        
        // Check if our new format mods are loaded
        if mod_list.contains(&"a.b.c".to_string()) {
            println!("Found a.b.c mod");
            if let Some(vars) = mod_manager.get_required_vars("a.b.c") {
                println!("a.b.c requires vars: {:?}", vars);
            }
        } else {
            println!("a.b.c mod not found");
        }
        
        if mod_list.contains(&"b.c.d".to_string()) {
            println!("Found b.c.d mod");
            if let Some(vars) = mod_manager.get_required_vars("b.c.d") {
                println!("b.c.d requires vars: {:?}", vars);
            }
        } else {
            println!("b.c.d mod not found");
        }
    }
}