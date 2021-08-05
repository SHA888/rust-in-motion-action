pub fn main() {
    let s = String::from("book");

    let pl = pluralize(s.clone());

    println!(
        "I have one {}, you have two {}",
        s,
        pl,
    );
}

// Add appropriate parameters, return values, and implementation to this function
fn pluralize(singular: String) -> String {
    
    singular + "s"
}