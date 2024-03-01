trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 12345,
        holder_name: String::from("Ali"),
        balance: 0.0,
    };

    let mut account2 = BankAccount {
        account_number: 54321,
        holder_name: String::from("Seyhan"),
        balance: 100.0,
    };

    account1.deposit(200.0);
    account2.withdraw(50.0);

    println!("{}'s account balance: {}", account1.holder_name ,account1.balance());
    println!("{}'s account balance: {}", account2.holder_name ,account2.balance());
}