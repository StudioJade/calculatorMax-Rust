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
        println!("---");
    }
}