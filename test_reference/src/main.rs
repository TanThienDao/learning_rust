fn main() {
    let ice_cream = "Cookie and cream";
    let dessert = ice_cream;

    println!("ice_cream: {ice_cream}");
    println!("dessert  : {dessert}");

    // Print the data pointer and length for each &str.
    println!("ice_cream ptr: {:p}, len: {}", ice_cream.as_ptr(), ice_cream.len());
    println!("dessert   ptr: {:p}, len: {}", dessert.as_ptr(), dessert.len());

    println!("same data pointer? {}", std::ptr::eq(ice_cream.as_ptr(), dessert.as_ptr()));
}
