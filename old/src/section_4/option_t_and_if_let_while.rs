pub fn option_t_and_if_let_while() {
    let x = 3.0;
    let y = 2.0;

    // Option -> Some(v) || None
    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot devide by 0")
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }

    while let Some(z) = result {
        println!("result = {}", z);
    }
}