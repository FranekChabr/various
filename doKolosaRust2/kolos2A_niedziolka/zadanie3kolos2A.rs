#[derive(Debug, Clone)]
struct Transaction {
    amount: f32,
    from_account: String,
    to_account: String,
}

struct BankAccount {
    account_number: String,
    history: Vec<Transaction>,
}

impl BankAccount {
    fn new(account_number: String) -> Self {
        BankAccount {
            account_number,
            history: Vec::new(),
        }
    }

    fn transaction(&mut self, t: Transaction) {
        if t.from_account == self.account_number || t.to_account == self.account_number {
            self.history.push(t);
        }
    }

    fn balance(&self) -> f32 {
        let mut balance = 0.0;
        for t in &self.history {
            if t.to_account == self.account_number {
                balance += t.amount;
            } else if t.from_account == self.account_number {
                balance -= t.amount;
            }
        }
        balance
    }

    fn last(&self) -> Option<&Transaction> {
        self.history.last()
    }
}

fn main() {
    let mut account = BankAccount::new("12345".to_string());

    let t1 = Transaction {
        amount: 100.0,
        from_account: "99999".to_string(),
        to_account: "12345".to_string(),
    };

    let t2 = Transaction {
        amount: 50.0,
        from_account: "12345".to_string(),
        to_account: "88888".to_string(),
    };

    account.transaction(t1);
    account.transaction(t2);

    println!("Balance: {}", account.balance());

    if let Some(last) = account.last() {
        println!(
            "Last transaction: from {} to {} amount {}",
            last.from_account, last.to_account, last.amount
        );
    } else {
        println!("No transactions yet.");
    }
}
