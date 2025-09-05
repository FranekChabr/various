fn pesel(s: &str) -> bool {
    // 11 znakow
    if s.len() != 11 {
        return false;
    }
    // same cyfry
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // cyfra kontrolna wg standardowych wag PESEL
    let digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let weights: [u32; 10] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];

    let sum: u32 = (0..10).map(|i| digits[i] * weights[i]).sum();
    let control = (10 - (sum % 10)) % 10;

    digits[10] == control
}

fn main() {
    // testy z kolokwim
    let valid = ["55030101193", "55030101230", "44051401458"];

    for p in valid {
        println!("{} -> {}", p, pesel(p)); // true
    }
    
    println!();

    let tests = [
        "12345678901",   // zla suma kontrolna
        "5503010119",    // krotki
        "55030101193a",  // litera
        "abcdefghijk",   // brak cyfr
    ];
    for p in tests {
        println!("{} -> {}", p, pesel(p)); // false
    }
}
