#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        if initial_balance < 0.0{
            panic!("Negative Balance");
        }
        let new_bank_account = BankAccount {
            balance : initial_balance,
        };
        new_bank_account
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount < 0.0{
            panic!("Negative Deposit");
        } else {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount >= self.balance{
            panic!("Withdraw Greater Than Balance")
        }
        self.balance -= amount;
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let new_account = BankAccount::new(200.00);
        assert_eq!(new_account.balance(), 200.00);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut new_account = BankAccount::new(200.00);
        new_account.deposit(500.25);
        assert_eq!(new_account.balance(), 700.25);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut new_account = BankAccount::new(200.00);
        new_account.withdraw(100.15);
        assert!((new_account.balance() - 99.85).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "Negative Balance")]
    fn test_create_new_account_neg_balance() {
        let _neg_account = BankAccount::new(-100.00);
    }

    #[test]
    #[should_panic(expected = "Negative Deposit")]
    fn test_deposit_neg_amount(){
        let mut test_account = BankAccount::new(100.00);
        test_account.deposit(-30.00);
    }

    #[test]
    #[should_panic(expected = "Withdraw Greater Than Balance")]
    fn test_withdraw_more_than_balance(){
        let mut dummy_account = BankAccount::new(500.34);
        dummy_account.withdraw(1000.10);
    }

    // Add more tests here
}
