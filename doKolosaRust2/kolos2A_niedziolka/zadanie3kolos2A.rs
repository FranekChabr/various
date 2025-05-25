#[derive(Debug, Clone)]
struct Transaction {
    amount: f32,
    sender: String,
    receiver: String, 
}

struct BankAccount {
    acc_number: String,
    history: Vec<Transaction>
}

impl BankAccount {
    fn new(acc_number: String) -> Self {
        BankAccount {
            acc_number,
            history: Vec::new(), 
        }
    }
    
    fn transaction(&mut self, t: Transaction) {
        if t.sender == self.acc_number || t.receiver == self.acc_number {
            self.history.push(t);   
        }
    }
    
    fn balance(&self) -> f32 {
        let mut balance = 0.0;
        for t in &self.history {
            if t.receiver == self.acc_number {
                balance += t.amount;
            } else if t.sender == self.acc_number {
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
        sender: "99999".to_string(),
        receiver: "12345".to_string(),
    };

    let t2 = Transaction {
        amount: 50.0,
        sender: "12345".to_string(),
        receiver: "88888".to_string(),
    };

    account.transaction(t1);
    account.transaction(t2);

    println!("Balance: {}", account.balance());

    if let Some(last) = account.last() {
        println!(
            "Last transaction: from {} to {} amount {}",
            last.sender, last.receiver, last.amount
        );
    } else {
        println!("No transactions yet.");
    }  
}



