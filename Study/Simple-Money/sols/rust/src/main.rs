mod sealerUnsealer;

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
    fn sprout(&self) -> Box<dyn Purse>;
    fn get_decr(&self) -> Box<SealedBox<Decr>>;
    fn deposit(&self, amount: i32, src: &dyn Purse);
}

struct PurseImpl {
    balance_slot: Box<i32>,
    decr: Decr,
}

pub trait Mint {
    fn make_purse(&self, balance: i32) -> Box<dyn Purse>;
    fn sprout(&self) -> Box<dyn Purse>
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

    fn sprout(&self) -> Box<dyn Purse> {
        self.mint.make_purse(0)
    }
    fn get_decr(&self) -> Box<SealedBox<Decr>> {
        self.mint.sealer.seal(self.decr.clone())
    }

    fn deposit(&self, amount: i32, src: &dyn Purse) {
        assert!(amount > 0);
        match self.mint.unsealer.unseal(src.get_decr()) {
            Some(d) => d(amount),
            None => panic!("Unsealing failed"),
        }
    }
}

impl Mint for MintImpl {
    fn make_purse(&self, balance: i32) -> Box<dyn Purse> {
        assert!(balance >= 0);

        let mut bslot: Box<i32> = Box::new(balance);
        let decr = Box::new(move |amt: i32| {
            assert!(amt <= *bslot); // or use Result type?
            *bslot -= amt;
        }) as Decr;
        Box::new(PurseImpl {
            balance_slot: bslot,
            decr,
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

fn main() {
    let (sealer, unsealer) = make_brand_pair("nickname".to_string());
    let obj = "42".to_string();
    let box_fn = sealer.seal(obj.clone());
    let result = unsealer.unseal(box_fn);
    match result {
        Ok(contents) => println!("Contents: {}", contents),
        Err(error) => println!("Error: {}", error),
    }
}
