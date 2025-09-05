#[derive(Clone, Debug, PartialEq)]
struct Book {
    title: String,
    author: String,
    year: u32,
    translator: Option<String>,
}

struct Library {
    books: Vec<(Book, bool)>, // (książka, czy dostępna)
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn push_book(&mut self, b: Book) {
        self.books.push((b, true));
    }

    fn find(&self, title: &str) -> Vec<Book> {
        self.books
            .iter()
            .filter(|(book, available)| book.title == title && *available)
            .map(|(book, _)| book.clone())
            .collect()
    }

    fn borrow(&mut self, b: Book) -> Result<Book, String> {
        for (book, available) in &mut self.books {
            if *available && *book == b {
                *available = false;
                return Ok(book.clone());
            }
        }
        Err("Książka niedostępna".to_string())
    }

    fn return_book(&mut self, b: Book) {
        for (book, available) in &mut self.books {
            if *book == b {
                *available = true;
            }
        }
    }
}

fn main() {
    let mut lib = Library::new();

    let b1 = Book {
        title: "Pan Tadeusz".to_string(),
        author: "Adam Mickiewicz".to_string(),
        year: 1834,
        translator: None,
    };

    let b2 = Book {
        title: "Dziady".to_string(),
        author: "Adam Mickiewicz".to_string(),
        year: 1823,
        translator: None,
    };

    lib.push_book(b1.clone());
    lib.push_book(b2.clone());

    println!("Szukam 'Pan Tadeusz': {:?}", lib.find("Pan Tadeusz"));

    match lib.borrow(b1.clone()) {
        Ok(book) => println!("Wypożyczono {:?}", book),
        Err(msg) => println!("Błąd: {}", msg),
    }

    println!("Szukam 'Pan Tadeusz': {:?}", lib.find("Pan Tadeusz"));

    lib.return_book(b1.clone());

    println!("Po oddaniu: {:?}", lib.find("Pan Tadeusz"));
}
