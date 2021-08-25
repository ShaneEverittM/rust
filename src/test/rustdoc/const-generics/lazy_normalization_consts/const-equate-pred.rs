#![crate_name = "foo"]
#![feature(const_generics, generic_const_exprs)]
#![allow(incomplete_features)]

// Checking if `Send` is implemented for `Hasher` requires us to evaluate a `ConstEquate` predicate,
// which previously caused an ICE.

pub struct Hasher<T> {
    cv_stack: T,
}

unsafe impl<T: Default> Send for Hasher<T> {}

// @has foo/struct.Foo.html
// @has - '//h3[@class="code-header in-band"]' 'impl Send for Foo'
pub struct Foo {
    hasher: Hasher<[u8; 3]>,
}
