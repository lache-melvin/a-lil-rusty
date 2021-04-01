fn main() {
    variables();
}

fn variables() { 
    // x is immutable
    let x = 4;

    // y is mutable
    let mut y = 5;
    println!("y before mutation: {}", y);
    y = 8;

    // constants are ALWAYS immutable
    // must always be typed
    // more for global/shared intel than anything else
    // usually defined in TRAIN/SCREAMING_SNAKE_CASE
    // also you can write big numbers like this 250_000 which is fun
    const MAX_CREDITS: u16 = 100;

    // shadowing allows us to redefine an immutable variable
    // good if we want to make some changes and then go back to being immutable
    // can change types with shadowing - you cannot change the type of a 
    //   mutable variable
    let z = 9;
    println!("z before shadowing: {}", z);
    let z = "sup"; // we have shadowed z!

    println!("FINAL x: {}, y: {}, MAX_CREDITS: {}, z: {}", x, y, MAX_CREDITS, z);
}
