// --- Struct ---

struct Policjant {
    imie: String, 
    nazwisko: String,
    wiek: u32,
    stanowisko: String,
}

fn print_policjant(p: &Policjant){
    println!("{} {} {} lat: {}", p.stanowisko, 
    p.imie, p.nazwisko, p.wiek);
}

fn main(){
    let policjant1 = Policjant {
        imie: String::from("Maciej"),
        nazwisko: String::from("Jeczydol"),
        wiek: 34,
        stanowisko: String::from("Oficer"),
    };
    
    print_policjant(&policjant1);
    
}

// --- Enum ---

enum Kolor{
    Czerwony,
    Niebieski,
    Zolty,
    Zielony,
}

fn print_kolor(k: Kolor){
    match k{
        Kolor::Czerwony => println!("Czerwony"),
        Kolor::Niebieski => println!("Niebieski"),
        Kolor::Zolty => println!("Zolty"),
        Kolor::Zielony => println!("Zielony"),
    }
}

fn main(){
    let kolorek = Kolor::Zielony; 
    print_kolor(kolorek);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Quit => println!("Koniec"),
    Message::Move { x, y } => println!("Przesuwam na ({}, {})", x, y),
    Message::Write(text) => println!("Piszę: {}", text),
    Message::ChangeColor(r, g, b) => println!("Kolor: rgb({}, {}, {})", r, g, b),
}

fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

let result = divide(10.0, 2.0);

match result {
    Some(value) => println!("Wynik: {}", value),
    None => println!("Nie można dzielić przez zero!"),
}


