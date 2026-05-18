/*
A generic is a type argument. it's a placeholder for a concrete type.
*/
fn add_5(value: i32) -> i32 {
    value + 5
}

fn identity_i32(value: i32) -> i32 {
    value
}
fn identity_bool(value: bool) -> bool {
    value
}
/// Generic type T
/// In Rust, Monimorphization is a compile-time process where polymorphic functions
/// are replace by many monomorphic functions for each unique instantiation.
fn identity<T>(value: T) -> T {
    value
}
#[derive(Debug)]
struct DeliSandwich;
#[test]
fn intro_to_generics(){
    println!("{}, {}, {}", add_5(10), identity_i32(10), identity_bool(true));
    println!("{} ", identity(5));
    println!("{} ", identity(13.25));
    println!("{} ", identity("string slice"));
    println!("{} ", identity(String::from("heap-allocated, growable string")));
    println!("{:?} ", identity(DeliSandwich));
}