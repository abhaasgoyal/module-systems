mod sealerUnsealer;
// use sealerUnsealer::{Sealer, SealedObject, Unsealer};

/*
struct Money {
    amount: u64,
}

impl SealedObject for Money {
    fn perform_operation(&self) {
        println!("Money: {}", self.amount);
    }
}

struct Mint {
    sealer: Sealer,

}

impl Mint {
    fn new() -> Mint {
        Mint {
            sealer: Sealer::new(),
        }
    }

    fn create_money(&mut self, amount: u64) -> String {
        let money = Money { amount };
        let sealed_money = Box::new(money);
        self.sealer.seal_object(sealed_money)
    }
}

struct Purse {
    capability: String,
    balance: u64,
}

impl Purse {
    fn new(capability: String) -> Purse {
        Purse {
            capability,
            balance: 0,
        }
    }

    fn sprout(&mut self, amount: u64, unsealer: &Unsealer) {
        // type is Dyn so could be broken
        match unsealer.unseal_object(&self.capability) {
            Some(obj) => {
                obj.perform_operation(); // Access the sealed object
                self.balance += amount;
                println!("Sprouting {} into the purse.", amount);
            }
            None => println!("Error: Invalid capability"),
        }
    }

    fn withdraw(&mut self, amount: u64, unsealer: &Unsealer) {
        match unsealer.unseal_object(&self.capability) {
            Some(obj) => {
                obj.perform_operation(); // Access the sealed object
                if self.balance >= amount {
                    self.balance -= amount;
                    println!("Withdrawing {} from the purse.", amount);
                } else {
                    println!("Error: Insufficient funds in the purse");
                }
            }
            None => println!("Error: Invalid capability"),
        }
    }


    /*
    fn deposit(&mut self, amount: u64, source_purse: &Purse, unsealer: &Unsealer) {
        match unsealer.unseal_object(&source_purse.capability) {
            Some(obj) => {
                obj.perform_operation(); // Access the sealed object
                if source_purse.balance >= amount {
                    source_purse.balance -= amount;
                    self.balance += amount;
                    println!("Depositing {} from the source purse.", amount);
                } else {
                    println!("Error: Insufficient funds in the source purse");
                }
            }
            None => println!("Error: Invalid capability for the source purse"),
        }
    }
    */

    fn get_balance(&self) -> u64 {
        self.balance
    }
}
*/
fn main() {
    /*
    let mut mint = Mint::new();
    let mint2 = Mint::new();
    let capability = mint.create_money(1000);

    let unsealer = Unsealer::new(&mint.sealer);
    let unsealer2 = Unsealer::new(&mint2.sealer);
    let mut purse = Purse::new(capability);

    purse.sprout(500, &unsealer);
    purse.get_balance();
    purse.withdraw(200, &unsealer);
    purse.withdraw(200, &unsealer2);
    */
    /*
    // Create a sealed object
    let obj = Box::new(MyObject { value: 42 });

    // Seal the object and get a capability
    let mut sealer = Sealer::new();
    let capability = sealer.seal_object(obj);

    // REVIEW: Pass the capability to another part of the system

    // Unseal the object using the capability
    let unsealer = Unsealer::new(&sealer);
    match unsealer.unseal_object(&capability) {
        Some(obj) => obj.perform_operation(),
        None => println!("Error: Invalid capability"),
    }
    */
}
