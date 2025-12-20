use calculator_max::calculator::mods::ModManager;

fn main() {
    let mut mod_manager = ModManager::new();
    if let Err(e) = mod_manager.load_mods() {
        eprintln!("Failed to load mods: {}", e);
    } else {
        println!("Successfully loaded mods");
        let mod_list = mod_manager.list_mods();
        println!("Available mods: {:?}", mod_list);

        for mod_name in mod_list {
            if let Some(vars) = mod_manager.get_required_vars(&mod_name) {
                println!("Mod '{}' requires vars: {:?}", mod_name, vars);
            } else {
                println!("Mod '{}' requires no vars", mod_name);
            }

            // Also print the mod details
            if let Some(mod_def) = mod_manager.get_mod(&mod_name) {
                println!("  Desc name: {:?}", mod_def.desc.name);
                println!("  Calc howto: {:?}", mod_def.calc.howto);
                println!("  Res res: {:?}", mod_def.res.res);
                println!("  Var needvars: {:?}", mod_def.var.needvars);
            }

            // Print raw mod content for debugging
            if mod_name == "a.b.c" || mod_name == "b.c.d" {
                println!("  Debug: Looking at mod {}", mod_name);
            }
        }
    }
}
