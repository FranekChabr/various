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

// =============================================================================================================================
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

// =============================================================================================================================
// --- impl ---

struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Associated function — nie używa `self`
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    // Metoda — używa `&self`
    fn say_hello(&self) {
        println!("Cześć, mam na imię {} i mam {} lat.", self.name, self.age);
    }

    // Metoda zwracająca coś
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    let p = Person::new("Ola", 20);
    p.say_hello();
    println!("Dorosły? {}", p.is_adult());
}
// =============================================================================================================================
// --- Derywacja cech (#[derive(...)]) ---
/*  Debug	Pozwala na wypisywanie {:?}
    Clone	Umożliwia kopiowanie (.clone())
    Copy	Umożliwia kopiowanie przez przypisanie (let b = a)
    PartialEq	Pozwala porównywać wartości (==, !=)
    Eq	Wersja PartialEq, ale dla pełnej równości
    Default	Tworzy domyślną wartość T::default()
*/ 

//#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    println!("{:?}", b);              // Debug
    println!("Czy równe? {}", a == b); // PartialEq
}


// =============================================================================================================================
// --- Vec<T> dynamiczna tablica ---
/*  Tworzenie pustego	     let v: Vec<i32> = Vec::new();
    Tworzenie z danymi	     let v = vec![1, 2, 3];
    Dodanie elementu	     v.push(4);
    Usunięcie elementu	     v.pop();
    Dostęp do elementu	     v[0], v.get(1)
    Iteracja po elementach	 for x in &v { ... }
    Długość	                 v.len()
    Usunięcie wszystkiego	 v.clear()
*/
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

fn main() {
    let mut fruits = vec!["apple", "banana", "orange"];

    fruits.push("grape");
    fruits.pop();

    for fruit in &fruits {
        println!("Owoc: {}", fruit);
    }

    println!("Liczba owoców: {}", fruits.len());
}

//Bezpieczny dostęp: get()
let v = vec![10, 20, 30];

match v.get(2) {
    Some(x) => println!("Element: {}", x),
    None => println!("Brak elementu"),
}

// =============================================================================================================================
//

