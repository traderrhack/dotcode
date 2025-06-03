use std::collections::BTreeMap;
// Imports the BTreeMap type from Rust's standard library. BTreeMap is an ordered map (key-value store) that keeps keys in sorted order.

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
// These are documentation comments describing the purpose of the module: to track account balances.

pub struct Pallet {
// Defines a public struct named `Pallet`. This struct will represent the balances module.
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<String, u128>,
// Declares a field named `balances` in the struct. It is a BTreeMap that maps account identifiers (as Strings) to their balances (as u128 integers).
}

impl Pallet {
// Begins an implementation block for the `Pallet` struct. This is where methods for `Pallet` are defined.
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
// Defines a public associated function `new` that returns a new instance of `Pallet`.
        Self { balances: BTreeMap::new() }
// Constructs a new `Pallet` with an empty BTreeMap for balances.
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
// Defines a public method `set_balance` that takes a mutable reference to self, a reference to an account identifier (`who`), and an amount. It sets the balance for the given account.
        self.balances.insert(who.clone(), amount);
// Inserts or updates the balance for the account `who` in the BTreeMap. `who.clone()` is used because BTreeMap requires ownership of the key.
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
// Defines a public method `balance` that takes a reference to self and a reference to an account identifier (`who`). Returns the balance for the account.
        *self.balances.get(who).unwrap_or(&0)
// Looks up the balance for `who` in the BTreeMap. If the account does not exist, returns 0.
    }

    	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
            // Defines a public method `transfer` that takes a mutable reference to self, the caller's account identifier, the recipient's account identifier, and the amount to transfer. It returns a Result indicating success or an error message.
                    if amount == 0 {
                        return Err("Amount must be non-zero");
                    }

                    let from_balance = self.balances.get(&caller).copied().unwrap_or(0);
                    if from_balance < amount {
                        return Err("Insufficient balance");
                    }

                    let to_balance = self.balances.get(&to).copied().unwrap_or(0);
                    let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow in recipient balance")?;
                    let new_from_balance = from_balance.checked_sub(amount).ok_or("Underflow in sender balance")?;

                    self.balances.insert(caller, new_from_balance);
                    self.balances.insert(to, new_to_balance);

		 

		Ok(())
	}
}

#[cfg(test)]
// This attribute tells the Rust compiler to only compile the following module when running tests.
mod tests {
// Defines a new module named `tests` for writing unit tests.
    #[test]
    // Marks the following function as a test case.
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
	    balances.set_balance(&"alice".to_string(), 100);
	    assert_eq!(balances.balance(&"alice".to_string()), 100);
	    assert_eq!(balances.balance(&"bob".to_string()), 0);ove!!!
    }
}