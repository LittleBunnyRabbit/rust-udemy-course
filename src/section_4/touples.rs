fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

pub fn touples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring

    let (a, b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);

    println!("{:?}", combined);
    println!("last element = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;

    println!("(({}, {}), ({}, {}))", c, d, e, f);

    let foo = (true, 42.1, 4);
    println!("{:?}", foo);

    let meaning = (42);
    println!("{:?}", meaning); // no brackets

    let meaning_single = (42,);
    println!("{:?}", meaning_single); // brackets
}