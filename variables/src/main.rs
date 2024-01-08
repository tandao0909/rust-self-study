fn main() {
    let x = {
        let x = 4;
        println!("Hello from the other side");
        x + 1
    };

    print!("The value of x is {x}");
}