fn email(s: &str) -> bool {
    // Sprawdź dozwolone znaki
    if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '@') {
        return false;
    }

    // Sprawdź pierwszy i ostatni znak
    let first = s.chars().next().unwrap();
    let last = s.chars().last().unwrap();
    if !first.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
        return false;
    }

    // Sprawdź, czy jest dokładnie jedna małpa
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let (local, domain) = (parts[0], parts[1]);

    // Obie części muszą być niepuste
    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // Część po małpie musi zawierać kropkę i nie może zaczynać się ani kończyć kropką
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    // Część przed małpą nie może zaczynać się ani kończyć kropką
    if local.starts_with('.') || local.ends_with('.') {
        return false;
    }

    true
}

fn main() {
    let test_cases = [
        "john.doe@example.com",     // true
        "john@examplecom",          // false (brak kropki po @)
        ".john@example.com",        // false (zaczyna się od .)
        "john.@example.com",        // false (kończy się na .)
        "john@exam@ple.com",        // false (więcej niż jedna @)
        "john@.com",                // false (domena zaczyna się od .)
        "john.doe@ex.ample.com",    // true
    ];

    for addr in test_cases {
        println!("{} -> {}", addr, email(addr));
    }
}
