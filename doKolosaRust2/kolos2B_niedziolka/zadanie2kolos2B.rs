struct Position {
    name: String,
    bought: bool,
}

// wlasna implementacja PartialEq – ignorujemy pole bought
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

struct ShoppingList {
    items: Vec<Position>,
}

impl ShoppingList {
    fn new() -> ShoppingList {
        ShoppingList { items: Vec::new() }
    }

    fn push(&mut self, p: Position) {
        self.items.push(p);
    }

    fn bought(&mut self, name: &str) {
        for item in &mut self.items {
            if item.name == name {
                item.bought = true;
            }
        }
    }

    fn print(self) {
        // najpierw niekupione
        for item in &self.items {
            if !item.bought {
                println!("{} (do kupienia)", item.name);
            }
        }
        // potem kupione
        for item in &self.items {
            if item.bought {
                println!("{} (kupione)", item.name);
            }
        }
    }
}

fn main() {
    let mut list = ShoppingList::new();

    list.push(Position {
        name: "Chleb".to_string(),
        bought: false,
    });
    list.push(Position {
        name: "Masło".to_string(),
        bought: false,
    });
    list.push(Position {
        name: "Mleko".to_string(),
        bought: false,
    });

    list.bought("Masło");

    list.print();
}
