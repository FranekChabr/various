fn email(s: &str) -> bool {
    // sprawdzanie znakow
    if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '@') {
        return false;
    }

    // pierwszy ostatni
    let first = s.chars().next().unwrap();
    let last = s.chars().last().unwrap();
    if !first.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
        return false;
    }

    // jedna malpa
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let (local, domain) = (parts[0], parts[1]);

    // opie czesci nie puste
    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // po @ musi zawierac kropke i nie moze zaczynac sei ani konczyc kropka
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    // przed @ nie moga zaczynac ani konczyc sie kropka
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

