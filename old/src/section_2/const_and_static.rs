const MEANING_OF_LIFE: u8 = 42; // no fixed address
static X: i32 = 123;
static mut Z: i32 = 123;

fn const_and_static() {
    println!("{}", MEANING_OF_LIFE);
    println!("{}", X);

    unsafe { // Because Z is mutable
        Z = 777;
        println!("{}", Z);
    }
}