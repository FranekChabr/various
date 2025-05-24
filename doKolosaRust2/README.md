## 1. `struct`
- Reprezentuje złożony typ danych (jak obiekt w OOP)
- Grupuje różne pola w jedną całość
- Stosowany do przechowywania danych: np. `User`, `Point`, `Car`

## 2. `enum`
- Reprezentuje typ z kilkoma wariantami
- Każdy wariant może zawierać własne dane
- Przydatny do modelowania stanów, opcji, błędów, np. `Option`, `Result`, `State`, `Message`

## 3. `impl`
- Dodaje metody i funkcje pomocnicze (associated functions) do `struct` i `enum`
- Umożliwia tworzenie konstruktorów (np. `::new()`)
- Dodaje logikę typu, np. `area()`, `is_valid()`

## 4. Derywacja cech (`#[derive(...)]`)
Automatycznie implementuje cechy biblioteczne jak:
- `Debug` – pozwala na `{:?}` w `println!`
- `Clone` – umożliwia kopiowanie danych
- `PartialEq` – umożliwia porównania `==`

Stosuje się do prostych `struct`/`enum`, które nie potrzebują własnej logiki przy tych cechach.

## 5. Implementacja cech (traits)
- Umożliwia dodanie zachowań zdefiniowanych przez cechy (np. `Display`, `Iterator`)
- Potrzebne np. do:
  - implementacji własnych typów, które mają zachowywać się jak inne (np. `Display` do ładnego wypisywania)
  - korzystania z generycznych funkcji

## 6. `Vec<T>`
- Dynamiczna tablica (wektor) – rośnie i maleje w trakcie działania programu
- Często używany kontener, np. `Vec<i32>`, `Vec<String>`
- Obsługuje metody jak: `.push()`, `.pop()`, `.len()`, `.iter()`

## 7. Iteratory biblioteczne
- Umożliwiają przechodzenie po kolekcjach (np. `for x in vec.iter()`)
- Obsługują metody: `.map()`, `.filter()`, `.collect()`, `.fold()`, `.any()`
- Pozwalają pisać zwięzły i wydajny kod funkcyjny

## 8. Lambdy (funkcje anonimowe)
- Krótkie funkcje „w locie”, np. `|x| x + 1`
- Używane z iteratorami: `.map(|x| x * 2)`
- Mogą mieć dostęp do zmiennych z otoczenia
