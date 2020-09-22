use std::mem;

fn fundamental_data_types() {
    // unsigned 0+
    let a:u8 = 123; // 8 bits
    println!("a = {}", a);

    let mut b:i8 = 0; // mutable
    println!("b = {}", b);

    b = 48;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    // i8 u8 i16 u16 i32 u32 i64 u64

    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, size = {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 8bytes or 64 bits, f64 (we also have f32)
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g:bool = true; // boolean
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
