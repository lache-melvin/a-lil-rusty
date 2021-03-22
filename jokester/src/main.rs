fn main() {
    println!("\nTell me a joke!");
    println!("\nWhat's the setup?");

    let mut setup = String::new();
    let mut punchline = String::new();

    std::io::stdin()
        .read_line(&mut setup)
        .expect("I couldn't quite catch that...");

    println!("\nAnd the punchline?");

    std::io::stdin()
        .read_line(&mut punchline)
        .expect("I couldn't quite catch that...");

    if rand::random() {
        println!("\nPAHAHAHHAHAHAHAHAHAHAHAHHHAAAAAAAAAAAAAAAHAHAHAHHAAAA THATS TOO FUNNY");
        println!("I'll try remember that one");
        println!("{} {}", setup, punchline);
    } else {
        println!("...I don't get it");
        println!("Well I do... it's just... not that funny");
        println!("Sorry");
    };
}
