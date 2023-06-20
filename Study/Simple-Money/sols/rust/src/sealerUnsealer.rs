use std::option::Option;

struct SealedBox<T> {
    shared: Option<T>,
}

impl<T> SealedBox<T> {
    fn share_content(&mut self, object: Option<T>) {
        self.shared = object;
    }
}

struct BrandSealer<T> {
    shared: Option<T>
}

impl<T> BrandSealer<T> {
    fn seal(&self, object: Option<T>) -> SealedBox<T> {
        let mut sealed_box = SealedBox { shared: None };
        sealed_box.share_content(object);
        sealed_box
    }
}

struct BrandUnsealer<T> {
    shared: Option<T>
}

impl<T> BrandUnsealer<T> {
    fn unseal(&self, mut sealed_box: SealedBox<T>) -> Option<T> {
        let shared = None;
        let mut result = None;
        sealed_box.share_content(shared);
        result
    }
}

struct BrandPair<T> {
    name: String,
    sealer: BrandSealer<T>,
    unsealer: BrandUnsealer<T>,
}

impl<T> BrandPair<T> {
    fn make_sealed_box(&self, object: Option<T>) -> SealedBox<T> {
        let mut sealed_box = SealedBox { shared: None };
        sealed_box.share_content(object);
        sealed_box
    }
}

fn make_brand_pair<T>(name: String) -> BrandPair<T> {
    let sealer = BrandSealer { shared: None};
    let unsealer = BrandUnsealer { shared: None};

    BrandPair {
        name,
        sealer,
        unsealer,
    }
}
