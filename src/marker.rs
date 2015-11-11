// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Primitive traits and marker types representing basic 'kinds' of types.
//!
//! Rust types can be classified in various useful ways according to
//! intrinsic properties of the type. These classifications, often called
//! 'kinds', are represented as traits.
//!
//! They cannot be implemented by user code, but are instead implemented
//! by the compiler automatically for the types to which they apply.
//!
//! Marker types are special types that are used with unsafe code to
//! inform the compiler of special constraints. Marker types should
//! only be needed when you are creating an abstraction that is
//! implemented using unsafe code. In that case, you may want to embed
//! some of the marker types below into your type.

/// Types able to be transferred across thread boundaries.
#[lang = "send"]
pub unsafe trait Send {
    // empty.
}

unsafe impl Send for .. { }

impl<T> !Send for *const T { }
impl<T> !Send for *mut T { }

/// Types with a constant size known at compile-time.
#[lang = "sized"]
#[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
pub trait Sized {
    // Empty.
}

/// Types that can be "unsized" to a dynamically sized type.
#[lang="unsize"]
pub trait Unsize<T> {
    // Empty.
}

pub trait Clone : Sized {
    /// Returns a copy of the value.
    ///
    /// # Examples
    ///
    /// ```
    /// let hello = "Hello"; // &str implements Clone
    ///
    /// assert_eq!("Hello", hello.clone());
    /// ```
    fn clone(&self) -> Self;
}

#[lang = "copy"]
pub trait Copy : Clone {
    // Empty.
}

#[lang = "sync"]
pub unsafe trait Sync {
    // Empty
}

unsafe impl Sync for .. { }

impl<T> !Sync for *const T { }
impl<T> !Sync for *mut T { }

#[lang = "phantom_data"]
pub struct PhantomData<T:?Sized>;

mod impls {
    use super::{Send, Sync, Sized};

    unsafe impl<'a, T: Sync + ?Sized> Send for &'a T {}
    unsafe impl<'a, T: Send + ?Sized> Send for &'a mut T {}
}

#[rustc_reflect_like]
#[allow(deprecated)]
pub trait Reflect {}

impl Reflect for .. { }
