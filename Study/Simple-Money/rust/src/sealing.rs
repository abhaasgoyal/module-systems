
#[derive(Clone)]
pub struct Brand {
    hint: String,
}

impl PartialEq for Brand {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

pub fn mk_brand_pair<T>(hint: String) -> (Box<dyn Sealer<T>>, Box<dyn Unsealer<T>>) {
    let brand = Box::new(Brand { hint: hint.clone() });
    let sealer = SealerImpl {
        brand: brand.clone(),
    };
    let unsealer = UnsealerImpl { brand };
    (Box::new(sealer), Box::new(unsealer))
}

pub struct SealedBox<T> {
    pub contents: T,
    brand: Box<Brand>,
}

impl<T> SealedBox<T> {
    pub fn new(contents: T, brand: Box<Brand>) -> Self {
        SealedBox { contents, brand }
    }
}

impl<T> std::fmt::Debug for SealedBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<sealed by {}>", self.brand.hint)
    }
}

pub trait Sealer<T> {
    fn seal(&self, it: T) -> Box<SealedBox<T>>;
    fn get_brand(&self) -> &Box<Brand>;
}

impl<T> std::fmt::Debug for Box<dyn Sealer<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<sealer for {}>", self.get_brand().hint)
    }
}

struct SealerImpl {
    brand: Box<Brand>,
}

impl<T> Sealer<T> for SealerImpl {
    fn seal(&self, it: T) -> Box<SealedBox<T>> {
        Box::new(SealedBox::new(it, self.brand.clone()))
    }

    fn get_brand(&self) -> &Box<Brand> {
        &self.brand
    }
}

pub trait Unsealer<T> {
    fn unseal(&self, sealed_box: Box<SealedBox<T>>) -> Option<T>;
}

struct UnsealerImpl {
    brand: Box<Brand>,
}

impl<T> Unsealer<T> for UnsealerImpl {
    fn unseal(&self, sealed_box: Box<SealedBox<T>>) -> Option<T> {
        if self.brand == sealed_box.brand {
            Some(sealed_box.contents)
        } else {
            None
        }
    }
}
