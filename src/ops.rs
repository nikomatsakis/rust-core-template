/// A version of the call operator that takes an immutable receiver.
#[lang = "fn"]
#[rustc_paren_sugar]
#[fundamental] // so that regex can rely that `&str: !FnMut`
pub trait Fn<Args> : FnMut<Args> {
    /// This is called when the call operator is used.
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

/// A version of the call operator that takes a mutable receiver.
#[lang = "fn_mut"]
#[rustc_paren_sugar]
#[fundamental] // so that regex can rely that `&str: !FnMut`
pub trait FnMut<Args> : FnOnce<Args> {
    /// This is called when the call operator is used.
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

/// A version of the call operator that takes a by-value receiver.
#[lang = "fn_once"]
#[rustc_paren_sugar]
#[fundamental] // so that regex can rely that `&str: !FnMut`
pub trait FnOnce<Args> {
    /// The returned type after the call operator is used.
    type Output;

    /// This is called when the call operator is used.
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
