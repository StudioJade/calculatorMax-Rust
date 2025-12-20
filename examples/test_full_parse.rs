use calculator_max::calculator::mods::{SimplifiedMod, Mod, ModDesc, ModVar, ModCalc, ModRes};

fn main() {
    let content = r#"[a.b.c]
name = "test_func"
type = "fun"
needs = ["x", "y"]
method = "x * y + 10"

[b.c.d]
name = "test_num"
type = "num"
res = 1.0
"#;

    // Split content into sections by empty lines or table headers
    let mut sections: Vec<String> = Vec::new();
    let mut current_section = String::new();
    
    for line in content.lines() {
        if line.starts_with('[') && !current_section.is_empty() {
            // New section started, save the previous one
            sections.push(current_section.clone());
            current_section.clear();
        }
        current_section.push_str(line);
        current_section.push('\n');
    }
    
    // Don't forget the last section
    if !current_section.is_empty() {
        sections.push(current_section);
    }
    
    println!("Found {} sections:", sections.len());
    for (i, section) in sections.iter().enumerate() {
        println!("Section {}:", i);
        println!("{}", section);
        
        // Extract mod ID from the first line which should be [mod.id]
        if let Some(first_line) = section.lines().next() {
            if first_line.starts_with('[') && first_line.ends_with(']') {
                let mod_id = first_line[1..first_line.len()-1].to_string();
                println!("  Mod ID: {}", mod_id);
                
                // Remove the first line (table header) and parse the rest as SimplifiedMod
                let content_without_header = section.lines().skip(1).collect::<Vec<_>>().join("\n");
                println!("  Content to parse: {}", content_without_header);
                
                // Try to parse the content as a single SimplifiedMod
                match toml::from_str::<SimplifiedMod>(&content_without_header) {
                    Ok(simplified_mod) => {
                        println!("  Successfully parsed:");
                        println!("    Name: {:?}", simplified_mod.name);
                        println!("    Type: {:?}", simplified_mod.mod_type);
                        println!("    Needs: {:?}", simplified_mod.needs);
                        println!("    Method: {:?}", simplified_mod.method);
                        println!("    Res: {:?}", simplified_mod.res);
                        
                        // Convert to legacy Mod structure for compatibility
                        let legacy_mod = Mod {
                            desc: ModDesc {
                                name: simplified_mod.name.clone(),
                            },
                            var: ModVar {
                                needvars: simplified_mod.needs.unwrap_or_else(Vec::new),
                            },
                            calc: ModCalc {
                                howto: simplified_mod.method,
                            },
                            res: ModRes {
                                res: simplified_mod.res,
                            },
                        };
                        
                        println!("  Converted to legacy Mod:");
                        println!("    Desc name: {:?}", legacy_mod.desc.name);
                        println!("    Calc howto: {:?}", legacy_mod.calc.howto);
                        println!("    Res res: {:?}", legacy_mod.res.res);
                        println!("    Var needvars: {:?}", legacy_mod.var.needvars);
                    }
                    Err(e) => {
                        eprintln!("  Failed to parse: {}", e);
                    }
                }
            }
        }
        
        println!("---");
    }
}