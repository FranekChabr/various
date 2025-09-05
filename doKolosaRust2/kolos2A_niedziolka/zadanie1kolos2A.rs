fn email(s: &str) -> bool {
    // czy znaki sie zgadzaja
    if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '@') {
        return false; 
    }
    // pierwszy i ostatni znak litery lub cyfry
    let first = s.chars().next().unwrap();
    let last = s.chars().last().unwrap();
    if !first.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
        return false; 
    }
    
    // zliczanie jednej malpy
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false; 
    }
    
    let (local, domain) = (parts[0], parts[1]);
    
    // obie czesci nie sa puste
    if local.is_empty() || domain.is_empty() {
        return false; 
    }

    // posiada kropeczni
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    // posiada kropeczni
    if local.starts_with('.') || local.ends_with('.') {
        return false;
    }
    
    true
}

fn main() {
    let test_cases = [
        "john.doe@example.com",        // true
        ".siemansonMihael.@gmail.com", // false
        "siemansonMihael@dupson.jp",   // true
        "ejopa@prytKom",               // false
        "_jpArmia@noSiema,xd",         // false
    ];
    
    for adresy in test_cases {
        println!("{} -> {}", adresy, email(adresy));
    }
}


