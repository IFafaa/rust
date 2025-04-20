
trait BankAccountOperations {
    fn create(titular: String, balance: f32) -> Self;
    fn deposit(&mut self, amount: f32);
    fn withdraw(&mut self, amount: f32);
    fn show_info(&self);
}
struct BankAccount {
    titular: String,
    balance: f32,
}

impl BankAccountOperations for BankAccount{
    fn create(titular: String, balance: f32) -> BankAccount {
        BankAccount { titular, balance }
    }

    fn deposit(&mut self, amount: f32) {
        println!("Depositing ${:.2}...", amount);
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f32) {
        println!("Withdrawing ${:.2}...", amount);
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds!");
        }
    }

    fn show_info(&self) {
        println!("Titular: {} - Balance: ${:.2}", self.titular, self.balance);
    }
}

pub fn bank_account() {
    let mut account = BankAccount::create(String::from("Fabricio Lima"), 1000.0);

    account.show_info();
    account.deposit(500.0);
    account.show_info();
    account.withdraw(200.0);
    account.show_info();
    account.withdraw(2000.0);
}
