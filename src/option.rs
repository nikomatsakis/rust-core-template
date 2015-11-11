use ops::{FnMut, FnOnce};

pub enum Option<T> {
    None,
    Some(T)
}

impl<T> Option<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        match self {
            Option::Some(x) => Option::Some(f(x)),
            Option::None => Option::None,
        }
    }
}
