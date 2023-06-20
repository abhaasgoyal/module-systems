mod sealerUnsealer;

mod sealing;
use sealing::{SealedBox, Sealer, Unsealer, mk_brand_pair};

type Decr = Box<dyn FnMut(i32)>;

// Mint Object
pub trait Mint {
    fn make_purse(&self, balance: i32) -> Box<dyn Purse>;
    fn sprout(&self) -> Box<dyn Purse>
}

struct MintImpl {
    name: String,
    sealer: Box<dyn Sealer<Decr>>,
    unsealer: Box<dyn Unsealer<Decr>>,
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


// Purse Object
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


impl Purse for PurseImpl {
    // TODO
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
