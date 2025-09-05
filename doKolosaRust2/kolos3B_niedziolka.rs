# Projekt Cargo: kolokwium3_b

Poniżej komplet plików. Skopiuj strukturę do nowego projektu (`cargo new kolokwium3_b`), dodaj pliki zgodnie ze ścieżkami i uruchom:

- testy: `cargo test`
- binarka 1 (Ocena): `cargo run --bin show_ocena`
- binarka 2 (Student): `cargo run --bin show_student`

---

**Cargo.toml**
```toml
[package]
name = "kolokwium3_b"
version = "0.1.0"
edition = "2021"

[dependencies]
```

---

**src/lib.rs**
```rust
pub mod ocena;
pub mod ocena_z_przedmiotu;
pub mod student;
```

---

**src/ocena.rs**
```rust
/// Typ `Ocena` – przechowuje wartość w skali 1..=5 w postaci u8.
/// Metody: new, jako_liczba, wieksza_niz
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ocena(u8);

impl Ocena {
    /// Tworzy nową ocenę. Dozwolone wartości: 1..=5.
    pub fn new(wartosc: u8) -> Self {
        assert!((1..=5).contains(&wartosc), "Ocena musi być w zakresie 1..=5");
        Self(wartosc)
    }

    /// Zwraca wartość liczbową oceny.
    pub fn jako_liczba(&self) -> u8 {
        self.0
    }

    /// True, jeśli `self` jest większa niż `inna`.
    pub fn wieksza_niz(&self, inna: &Ocena) -> bool {
        self.0 > inna.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tworzenie_poprawnej_oceny() {
        let o = Ocena::new(4);
        assert_eq!(o.jako_liczba(), 4);
    }

    #[test]
    #[should_panic]
    fn odrzuca_nieprawidlowa_ocene() {
        let _ = Ocena::new(0); // poza zakresem
    }

    #[test]
    fn porownywanie_ocen() {
        let a = Ocena::new(5);
        let b = Ocena::new(3);
        assert!(a.wieksza_niz(&b));
        assert!(!b.wieksza_niz(&a));
    }
}
```

---

**src/ocena_z_przedmiotu.rs**
```rust
use crate::ocena::Ocena;

/// Powiązanie oceny z nazwą przedmiotu.
/// Metody: new, zwroc_ocene, zwroc_przedmiot, zmien_przedmiot, zmien_ocene
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcenaZPrzedmiotu {
    przedmiot: String,
    ocena: Ocena,
}

impl OcenaZPrzedmiotu {
    pub fn new<S: Into<String>>(przedmiot: S, ocena: Ocena) -> Self {
        Self { przedmiot: przedmiot.into(), ocena }
    }

    pub fn zwroc_ocene(&self) -> Ocena {
        self.ocena
    }

    pub fn zwroc_przedmiot(&self) -> &str {
        &self.przedmiot
    }

    pub fn zmien_przedmiot<S: Into<String>>(&mut self, nowy: S) {
        self.przedmiot = nowy.into();
    }

    pub fn zmien_ocene(&mut self, nowa: Ocena) {
        self.ocena = nowa;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tworzenie_i_odczyt() {
        let o = OcenaZPrzedmiotu::new("Matematyka", Ocena::new(5));
        assert_eq!(o.zwroc_przedmiot(), "Matematyka");
        assert_eq!(o.zwroc_ocene().jako_liczba(), 5);
    }

    #[test]
    fn modyfikacje() {
        let mut o = OcenaZPrzedmiotu::new("Fizyka", Ocena::new(3));
        o.zmien_przedmiot("Infa");
        o.zmien_ocene(Ocena::new(4));
        assert_eq!(o.zwroc_przedmiot(), "Infa");
        assert_eq!(o.zwroc_ocene().jako_liczba(), 4);
    }
}
```

---

**src/student.rs**
```rust
use crate::ocena::Ocena;
use crate::ocena_z_przedmiotu::OcenaZPrzedmiotu;

/// Student przechowuje wiele `OcenaZPrzedmiotu` i udostępnia operacje:
/// - nowy student,
/// - dodanie oceny z przedmiotu,
/// - liczba ocen,
/// - średnia ocen,
/// - zwrócenie oceny z danego przedmiotu,
/// - zwrócenie ocen w postaci napisu.
#[derive(Debug, Default)]
pub struct Student {
    pub imie: String,
    oceny: Vec<OcenaZPrzedmiotu>,
}

impl Student {
    pub fn new<S: Into<String>>(imie: S) -> Self {
        Self { imie: imie.into(), oceny: Vec::new() }
    }

    pub fn dodaj_ocene<S: Into<String>>(&mut self, przedmiot: S, ocena: Ocena) {
        self.oceny.push(OcenaZPrzedmiotu::new(przedmiot, ocena));
    }

    pub fn liczba_ocen(&self) -> usize {
        self.oceny.len()
    }

    pub fn srednia(&self) -> Option<f32> {
        if self.oceny.is_empty() { return None; }
        let suma: u32 = self.oceny.iter().map(|o| o.zwroc_ocene().jako_liczba() as u32).sum();
        Some(suma as f32 / self.oceny.len() as f32)
    }

    pub fn ocena_z_przedmiotu(&self, przedmiot: &str) -> Option<Ocena> {
        self.oceny
            .iter()
            .find(|o| o.zwroc_przedmiot() == przedmiot)
            .map(|o| o.zwroc_ocene())
    }

    /// Zwraca wszystkie oceny w postaci prostego napisu, np.
    /// "Matematyka: 5, Fizyka: 3"
    pub fn oceny_as_string(&self) -> String {
        self.oceny
            .iter()
            .map(|o| format!("{}: {}", o.zwroc_przedmiot(), o.zwroc_ocene().jako_liczba()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dodawanie_i_liczenie() {
        let mut s = Student::new("Ala");
        s.dodaj_ocene("Matma", Ocena::new(5));
        s.dodaj_ocene("Fizyka", Ocena::new(3));
        assert_eq!(s.liczba_ocen(), 2);
        assert!(matches!(s.srednia(), Some(x) if (x - 4.0).abs() < 1e-6));
    }

    #[test]
    fn wyszukiwanie_po_przedmiocie() {
        let mut s = Student::new("Ola");
        s.dodaj_ocene("Infa", Ocena::new(4));
        assert_eq!(s.ocena_z_przedmiotu("Infa").unwrap().jako_liczba(), 4);
        assert!(s.ocena_z_przedmiotu("Historia").is_none());
    }

    #[test]
    fn formatowanie_ocen() {
        let mut s = Student::new("Jan");
        s.dodaj_ocene("Matma", Ocena::new(5));
        s.dodaj_ocene("Polski", Ocena::new(4));
        let txt = s.oceny_as_string();
        assert!(txt.contains("Matma: 5"));
        assert!(txt.contains("Polski: 4"));
    }
}
```

---

**src/bin/show_ocena.rs** – prezentacja typu 1 (Ocena)
```rust
use kolokwium3_b::ocena::Ocena;

fn main() {
    let a = Ocena::new(5);
    let b = Ocena::new(3);

    println!("Ocena a = {}", a.jako_liczba());
    println!("Ocena b = {}", b.jako_liczba());
    println!("Czy a > b? {}", a.wieksza_niz(&b));
}
```

---

**src/bin/show_student.rs** – prezentacja typu 3 (Student)
```rust
use kolokwium3_b::ocena::Ocena;
use kolokwium3_b::student::Student;

fn main() {
    let mut s = Student::new("Kasia");
    s.dodaj_ocene("Matematyka", Ocena::new(5));
    s.dodaj_ocene("Fizyka", Ocena::new(3));
    s.dodaj_ocene("Informatyka", Ocena::new(4));

    println!("Student: {}", s.imie);
    println!("Liczba ocen: {}", s.liczba_ocen());
    println!("Średnia: {}", s.srednia().unwrap());
    println!("Oceny: {}", s.oceny_as_string());

    if let Some(o) = s.ocena_z_przedmiotu("Fizyka") {
        println!("Ocena z Fizyki: {}", o.jako_liczba());
    }
}
