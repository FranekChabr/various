# Zakres zagadnień na kolokwium nr 2

## 1. `struct`
- Reprezentuje złożony typ danych (podobny do obiektu w OOP).  
- Grupuje różne pola w jedną całość.  
- Stosowany do przechowywania danych, np. `User`, `Point`, `Car`.  

**Przykład:**
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 7 };
    println!("Punkt: ({}, {})", p.x, p.y);
}
```

## 2. enum
- Umożliwia definiowanie typu z wieloma możliwymi wariantami.
- Często używany do reprezentowania stanów, opcji, błędów.

**Przykład:**
```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;
    match dir {
        Direction::North => println!("Idziesz na północ"),
        _ => println!("Inny kierunek"),
    }
}
```

## 3. impl
- Służy do implementacji metod dla struct i enum.
- Pozwala dodać funkcje związane z danym typem.

Przykład:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 5, height: 10 };
    println!("Pole: {}", rect.area());
}
```

## 4. Derywacja i implementacja cech bibliotecznych
- Derywacja (derive): automatyczne implementowanie cech (np. Debug, Clone, PartialEq).
- Implementacja: własne definiowanie zachowania cechy (trait).

Przykład:
```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let u1 = User { name: "Ala".to_string(), age: 20 };
    println!("{:?}", u1);
}
```

## 5. Vec
- Dynamiczna tablica o zmiennym rozmiarze.
- Często używana zamiast tablic statycznych.

Przykład:
```rust
fn main() {
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("{:?}", numbers);
}
```

## 6. Użycie iteratorów bibliotecznych
- Iteratory pozwalają przetwarzać kolekcje w sposób deklaratywny.
- Przykłady: map, filter, collect.

Przykład:
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();

    println!("{:?}", squares);
}
```

## 7. Lambdy (funkcje anonimowe)
- Funkcje bez nazwy, definiowane „w locie”.
- Często używane z iteratorami.

Przykład:
```rust
fn main() {
    let add = |a: i32, b: i32| a + b;
    println!("3 + 5 = {}", add(3, 5));

    let numbers = vec![1, 2, 3];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
}
```

# Zakres zagadnień na kolokwium nr 3

## 1. Rustowe podejście do przekazywania danych, przeciążania, obiektowości itd.
- Rust stawia na **własność (ownership)**, **pożyczanie (borrowing)** i **czas życia (lifetimes)** zamiast klasycznej obiektowości.  
- Zamiast dziedziczenia wykorzystuje **kompozycję** i **cechy (`traits`)**.  
- Dzięki temu unika problemów z pamięcią i pozwala pisać bezpieczny kod.  

**Przykład:**
```rust
struct User {
    name: String,
}

impl User {
    fn greet(&self) {
        println!("Cześć, {}!", self.name);
    }
}

fn main() {
    let u = User { name: "Ala".to_string() };
    u.greet(); // borrowing &self
}
```

## 2. Aplikacje wielomodułowe
- Kod można dzielić na moduły (mod) i pliki, aby zachować porządek.
- Moduły mogą być publiczne (pub).

Przykład (plik main.rs + osobny plik):
```rust
mod greetings;

fn main() {
    greetings::hello();
}
```


Plik greetings.rs:
```rust
pub fn hello() {
    println!("Hello from module!");
}
```

## 3. Przeciążanie operatorów
- Realizowane przez implementację odpowiednich cech (traits) np. Add, Sub, Mul.

Przykład:
```rust
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("({}, {})", p3.x, p3.y);
}
```

## 4. Typy i funkcje generyczne
- Generyki umożliwiają tworzenie uniwersalnych struktur i funkcji.
- Oparte o trait bounds do ograniczania typów.

Przykład:
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let nums = vec![10, 20, 5];
    println!("Największy: {}", largest(&nums));
}
```

## 5. Wbudowane testowanie
- Rust ma wbudowany framework testowy (cargo test).
- Testy oznacza się atrybutem #[test].

Przykład:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

## 6. Box i dynamiczne typowanie
- Box<T> służy do przechowywania danych na stercie (heap).
- Umożliwia pracę z typami o nieznanym rozmiarze w czasie kompilacji.
- W połączeniu z dyn Trait pozwala na polimorfizm.

Przykład:
```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

fn main() {
    let c: Box<dyn Shape> = Box::new(Circle { r: 5.0 });
    println!("Pole koła: {}", c.area());
}
```
