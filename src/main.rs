mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
	}
}

fn main() {
	println!("🚀 Starting Polkadot Runtime Demo!");

	// Create a new runtime instance
	let mut runtime = Runtime::new();

	println!("\n=== System Pallet Demo ===");
	println!("Initial block number: {}", runtime.system.block_number());

	// Increment block number a few times
	runtime.system.inc_block_number();
	println!("After increment: {}", runtime.system.block_number());

	runtime.system.inc_block_number();
	println!("After another increment: {}", runtime.system.block_number());

	// Test nonce functionality
	let alice = "alice".to_string();
	let bob = "bob".to_string();

	runtime.system.inc_nonce(&alice);
	runtime.system.inc_nonce(&alice);
	runtime.system.inc_nonce(&bob);

	println!("Alice has made transactions (nonce increments)");
	println!("Bob has made transactions (nonce increments)");

	println!("\n=== Balances Pallet Demo ===");

	// Set initial balances
	runtime.balances.set_balance(&alice, 1000);
	runtime.balances.set_balance(&bob, 500);

	println!("Alice's balance: {}", runtime.balances.balance(&alice));
	println!("Bob's balance: {}", runtime.balances.balance(&bob));

	// Test transfer
	println!("\n--- Transfer Test ---");
	println!("Transferring 200 from Alice to Bob...");

	match runtime.balances.transfer(alice.clone(), bob.clone(), 200) {
		Ok(()) => {
			println!("✅ Transfer successful!");
			println!("Alice's new balance: {}", runtime.balances.balance(&alice));
			println!("Bob's new balance: {}", runtime.balances.balance(&bob));
		}
		Err(e) => println!("❌ Transfer failed: {}", e),
	}

	// Test insufficient balance
	println!("\n--- Testing Insufficient Balance ---");
	println!("Trying to transfer 2000 from Alice to Bob (should fail)...");

	match runtime.balances.transfer(alice.clone(), bob.clone(), 2000) {
		Ok(()) => println!("✅ Transfer successful!"),
		Err(e) => println!("❌ Transfer failed as expected: {}", e),
	}

	// Final state
	println!("\n=== Final State ===");
	println!("Final block number: {}", runtime.system.block_number());
	println!("Alice's final balance: {}", runtime.balances.balance(&alice));
	println!("Bob's final balance: {}", runtime.balances.balance(&bob));

	println!("\n🎉 Runtime demo completed successfully!");
}