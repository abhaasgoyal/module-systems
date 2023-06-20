mod sealerUnsealer;
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
mod sealing;
use sealing::{SealedBox, Sealer, Unsealer, mk_brand_pair};

#[cfg(test)]
mod sealing_tests {
    use super::sealing::*;

    #[test]
    fn simple_sealing() {
        let (s, u) = mk_brand_pair("bob".to_string());
        let sekret = 42;
        match u.unseal(s.seal(sekret)) {
            Some(_) => (),
            None => panic!("Unsealing failed"),
        }
    }
}


type Decr = Box<dyn FnMut(i32)>;

pub trait Purse {
    fn get_balance(&self) -> i32;
    // fn sprout(&self) -> Box<dyn Purse>;
    // fn get_decr(&self) -> Box<SealedBox<Decr>>;
    // fn deposit(&self, amount: i32, src: &dyn Purse);
}

struct PurseImpl {
    balance_slot: Box<i32>,
    // decr: Decr,
}

pub trait Mint {
    fn make_purse(&self, balance: i32) -> Box<dyn Purse>;
    // fn sprout(&self) -> Box<dyn Purse>
}

struct MintImpl {
    name: String,
    sealer: Box<dyn Sealer<Decr>>,
    unsealer: Box<dyn Unsealer<Decr>>,
}

impl Purse for PurseImpl {
    fn get_balance(&self) -> i32 {
        *self.balance_slot
    }

    // fn sprout(&self) -> Box<dyn Purse> {
    //     self.mint.make_purse(0)
    // }
    // fn get_decr(&self) -> Box<SealedBox<Decr>> {
    //     self.mint.sealer.seal(self.decr.clone())
    // }

    // fn deposit(&self, amount: i32, src: &dyn Purse) {
    //     assert!(amount > 0);
    //     match self.mint.unsealer.unseal(src.get_decr()) {
    //         Some(d) => d(amount),
    //         None => panic!("Unsealing failed"),
    //     }
    // }
}

impl Mint for MintImpl {
    fn make_purse(&self, balance: i32) -> Box<dyn Purse> {
        assert!(balance >= 0);

        let mut bslot: Box<i32> = Box::new(balance);
        // let decr = Box::new(move |amt: i32| {
        //     assert!(amt <= *bslot); // or use Result type?
        //     *bslot -= amt;
        // }) as Decr;
        Box::new(PurseImpl {
            balance_slot: bslot,
            // decr,
        })
    }
}

pub fn make_mint(name: String) -> Box<dyn Mint> {
    let (sealer, unsealer) = mk_brand_pair(name.clone());
    Box::new(MintImpl {
        name,
        sealer,
        unsealer,
    })
}



// #[cfg(test)]
// mod money_tests {
//     use super::mint::*;

//     #[test]
//     fn ode() {
//         let carol_mint = make_mint("Carol".to_string());

//         let alice_main_purse = carol_mint.make_purse(1000);
//         assert_eq!(alice_main_purse.get_balance(), 1000);

//         let bob_main_purse = carol_mint.make_purse(0);
//         assert_eq!(bob_main_purse.get_balance(), 0);

//         bob_main_purse.deposit(10, &*alice_main_purse);
//         assert_eq!(bob_main_purse.get_balance(), 10);
//         assert_eq!(alice_main_purse.get_balance(), 990);
//     }

//     #[test]
//     #[should_panic]
//     fn illegal_balance() {
//         let carol_mint = make_mint("Carol".to_string());

//         let _bad1 = carol_mint.make_purse(-5);
//     }
// }

fn main() {
    // let (s, u) = sealing::mk_brand_pair("bob".to_string()); //
    // let sekret = 42;
    // match u.unseal(s.seal(sekret)) {
    //     Some(i) => println!("seal, unseal: {:?}", i),
    //     None => println!("lose!"),
    // }
}
