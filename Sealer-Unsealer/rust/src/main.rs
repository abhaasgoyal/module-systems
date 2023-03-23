use std::collections::HashMap;

trait SealedObject {
    fn perform_operation(&self);
}

// Creates capability to access a specific object
struct Sealer {
    objects: HashMap<String, Box<dyn SealedObject>>,
}

impl Sealer {
    fn new() -> Sealer {
        Sealer {
            objects: HashMap::new(),
        }
    }

    fn seal_object(&mut self, obj: Box<dyn SealedObject>) -> String {
        // Generate a unique token to use as the capability using PRNG
        let token = generate_unique_token();

        // Store the object and associated token
        self.objects.insert(token.clone(), obj);

        // Return the token as the capability
        token
    }
}

struct Unsealer {
    objects: HashMap<String, Box<dyn SealedObject>>,
}

impl Unsealer {
    fn new(sealer: &Sealer) -> Unsealer {
        Unsealer {
            objects: sealer.objects.clone(),
        }
    }

    // Check capabilities and returns the sealed object
    fn unseal_object(&self, capability: &str) -> Option<&Box<dyn SealedObject>> {
        // Check that the capability is valid and exists in HashMap
        self.objects.get(capability)
    }
}

// Define a struct that implements the SealedObject trait
struct MyObject {
    value: i32,
}

impl SealedObject for MyObject {
    fn perform_operation(&self) {
        println!("MyObject: {}", self.value);
    }
}

// Helper function to generate a unique token
fn generate_unique_token() -> String {
    // TODO: Implement PRNG
    String::from("12345")
}

fn main() {
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
}
