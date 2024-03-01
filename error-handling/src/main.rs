trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: u32, holder_name: &str) -> BankAccount {
        BankAccount {
            account_number,
            holder_name: holder_name.to_string(),
            balance: 0.0,
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be greater than zero".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdraw amount must be greater than zero".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount::new(123456, "Ali");
    let mut account2 = BankAccount::new(654321, "Seyhan");

    match account1.deposit(1000.0) {
        Ok(()) => println!("Deposit successful."),
        Err(err) => println!("Deposit error: {}", err),
    }

    match account2.deposit(1000.0) {
        Ok(()) => println!("Deposit successful."),
        Err(err) => println!("Deposit error: {}", err),
    }

    match account2.withdraw(500.0) {
        Ok(()) => println!("Withdrawal successful."),
        Err(err) => println!("Withdrawal error: {}", err),
    }

    println!("Account 1 balance: {}", account1.balance());
    println!("Account 2 balance: {}", account2.balance());
}
