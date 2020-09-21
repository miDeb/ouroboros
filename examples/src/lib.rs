use ouroboros::self_referencing;

#[cfg(test)]
mod ok_tests;

#[self_referencing]
/// A simple struct which contains a `Box<i32>` and a `&'this i32`.
pub struct BoxAndRef {
    data: Box<i32>,
    #[borrows(data)]
    data_ref: &'this i32,
}

#[self_referencing(chain_hack)]
/// A chain of references, where c references b which references a. This is an example of a struct
/// which requires using [chain_hack](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#using-chain_hack)
/// as of the time this was written.
pub struct ChainHack {
    a: Box<i32>,
    #[borrows(a)]
    b: Box<&'this i32>,
    #[borrows(b)]
    c: Box<&'this i32>,
}

#[self_referencing]
/// The example provided in the documentation.
pub struct DocumentationExample {
    int_data: Box<i32>,
    float_data: Box<f32>,
    #[borrows(int_data)]
    int_reference: &'this i32,
    #[borrows(mut float_data)]
    float_reference: &'this mut f32,
}