#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

const MEANING_OF_LIFE: u8 = 42; // global, no fixed address, replaces with the value
static mut Z: i32 = 123; // global mut

pub fn core_data_types() {
    let a: u8 = 123; // u = unsigned, 8b, 0 - 255
    println!("a = {}", a); // immutable

    // u = unsigned, 0 to 2^n-1
    // i = signed, -2^(n-1) ... 2^(n-1)-1
    let mut b: i8 = 0; // -128 ... 127
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789; // i32 = 32b = 4B
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    // u8, u16, ..., i8, i16, ...

    // usize and isize - native to the processor
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d: char = 'x'; 
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // f32, f64 (IEEE754) signed!

    let e: f32 = 2.5;
    println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));

    let g: bool = false; // true
    println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));

}

pub fn operators() {
    let mut a = 2 + 3 * 4; // +-*/
    println!("{}", a);
    a = a + 1; // -- ++ are not supported
    a -= 2; // this works

    println!("remainder of {} / {} = {}", a, 3, a % 3);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR, & AND, ^ XOR, ! NOR
                   // 01 OR 10 = 11 = 3_10
    println!("1 | 2 = {}", c);
    let two_to_10 = 1 << 10; 
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true

    let x = 5;
    let x_is_5 = x == 5; // true

}

pub fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    // println!("outiside b = {}", b); // No go
    println!("outside, a = {}", a);
}

pub fn const_and_static() {
    unsafe { // I promise that im working safe with the variable
        println!("{}", Z);
    }
}

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin(); // on stack
    let p2 = Box::new(origin()); // on heap, stores just the address on the stack

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2; // p3 is on the stack, unboxing

    println!("p3 takes up {} bytes", mem::size_of_val(&p3));
}