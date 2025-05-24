/*
1. struct
* Reprezentuje złożony typ danych (jak obiekt w OOP)
* Grupuje różne pola w jedną całość
* Stosowany do przechowywania danych: np. User, Point, Car

2. enum
* Reprezentuje typ z kilkoma wariantami
* Każdy wariant może zawierać własne dane
* Przydatny do modelowania stanów, opcji, błędów, np. Option, Result, State, Message

3. impl
* Dodaje metody i funkcje pomocnicze (associated functions) do struct i enum
* Umożliwia tworzenie konstruktorów (::new())
* Dodaje logikę typu, np. area(), is_valid()

4. Derywacja cech (np. #[derive(...)])
Automatycznie implementuje cechy biblioteczne jak:
  * Debug – pozwala na {:?} w println!
  * Clone – umożliwia kopiowanie danych
  * PartialEq – umożliwia porównania ==
Stosuje się do prostych struct/enum, które nie potrzebują własnej logiki przy tych cechach

5. Implementacja cech (traits)
* Umożliwia dodanie zachowań zdefiniowanych przez cechy (np. Display, Iterator)
Potrzebne np. do:
  * implementacji własnych typów, które mają zachowywać się jak inne (np. Display do ładnego wypisywania)
korzystania z generycznych funkcji

6. Vec<T>
* Dynamiczna tablica (wektor) – rośnie i maleje w trakcie działania programu
* Często używany kontener, np. Vec<i32>, Vec<String>
* Obsługuje metody jak: .push(), .pop(), .len(), .iter()

7. Iteratory biblioteczne
* Umożliwiają przechodzenie po kolekcjach (np. for x in vec.iter())
* Obsługują metody: .map(), .filter(), .collect(), .fold(), .any()
* Pozwalają pisać zwięzły i wydajny kod funkcyjny

8. Lambdy (funkcje anonimowe)
* Krótkie funkcje „w locie”, np. |x| x + 1
* Używane z iteratorami: .map(|x| x * 2)
* Mogą mieć dostęp do zmiennych z otoczenia
*/
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


