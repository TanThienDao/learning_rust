/// The iter_mut method will create an iterator that yields mutable reference to the collection's elements.


fn main() {
    let mut flavours = [
        String::from("Blue"),
        String::from("White"),
        String::from("Red"),
    ];

    let iter_mut = flavours.iter_mut();

    for color in iter_mut {
        color.push_str("berry");
    }
    println!("flavours: {:?}", flavours);
    for color in &mut flavours {
        color.push_str(" Ha ha");
    }
    println!("flavours: {:?}", flavours);

    // With certain data types, we still need to use the dereference operator, the asterisk '*' to
    // access the value that the mutable reference points to. This is because some types,
    // like String, are not Copy types and cannot be implicitly copied.
    // Therefore, we need to dereference the mutable reference to access the underlying value and modify it.

    let mut shool_grades = [85,90, 95, 100];

    for grade in &mut shool_grades {
        *grade -= 2;
    }

    println!("shool_grades: {:?}", shool_grades);

    // There are 3 syntax iterators in Rust: iter(), iter_mut(), and into_iter(). The iter() method creates an iterator that yields immutable references to the collection's elements, allowing you to read the values without modifying them. The iter_mut() method creates an iterator that yields mutable references to the collection's elements, allowing you to modify the values in place. The into_iter() method consumes the collection and creates an iterator that yields owned values, transferring ownership of the elements to the iterator.

    // OWNERSHIP RULES:
    // 1. The iter() method does not take ownership of the collection, allowing you to continue using the collection after the iteration.
    // 2. The iter_mut() method also does not take ownership of the collection, allowing you to continue using the collection after the iteration.
    // 3. The into_iter() method takes ownership of the collection, consuming it and transferring ownership of the elements to the iterator.
    // for value in collection

    //OWNERSHIP
    // for value in collection
    // for value in collection.into_iter()

    // IMMUTABLE REFERENCES
    // for value in &collection
    // for value in collection.iter()

    // MUTABLE REFERENCES
    // for value in &mut collection
    // for value in collection.iter_mut()

}