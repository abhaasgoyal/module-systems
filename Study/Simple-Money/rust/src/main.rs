use std::cell::RefCell;
use std::rc::Rc;

struct BrandPair<'a, T> {
    shared: Rc<RefCell<Option<T>>>,
    lifetime: std::marker::PhantomData<&'a T>,
}

impl<'a, T> BrandPair<'a, T> {
    fn make_sealed_box(&self, obj: T) -> Box<dyn FnOnce() -> () + 'a> {
        let shared_clone = Rc::clone(&self.shared);
        Box::new(move || {
            *shared_clone.borrow_mut() = Some(obj);
        })
    }
}

struct Sealer<'a, T> {
    shared: Rc<RefCell<Option<T>>>,
    lifetime: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Sealer<'a, T> {
    fn seal(&self, obj: T) -> Box<dyn FnOnce() -> () + 'a> {
        let shared_clone = Rc::clone(&self.shared);
        Box::new(move || {
            *shared_clone.borrow_mut() = Some(obj);
        })
    }
}

struct Unsealer<'a, T> {
    shared: Rc<RefCell<Option<T>>>,
    lifetime: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Unsealer<'a, T> {
    fn unseal(&self, box_fn: Box<dyn FnOnce() -> () + 'a>) -> Result<T, &'static str> {
        *self.shared.borrow_mut() = None;
        box_fn();
        let contents = self.shared.borrow_mut().take();
        match contents {
            Some(value) => Ok(value),
            None => Err("invalid box"),
        }
    }
}

fn make_brand_pair<'a, T>(nickname: T) -> (Sealer<'a, T>, Unsealer<'a, T>) {
    let shared = Rc::new(RefCell::new(None));
    let brand_pair = BrandPair {
        shared: Rc::clone(&shared),
        lifetime: std::marker::PhantomData,
    };
    let sealer = Sealer {
        shared: Rc::clone(&shared),
        lifetime: std::marker::PhantomData,
    };
    let unsealer = Unsealer {
        shared: Rc::clone(&shared),
        lifetime: std::marker::PhantomData,
    };
    (sealer, unsealer)
}

// TODO: Implement Mint and Purse

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
