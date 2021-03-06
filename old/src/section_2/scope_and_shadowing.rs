fn scope_and_shadowing() {
    let a = 123;
    let a = 1234;

    {
        let b = 456;
        println!("inside, b = {}", b); // 456  

        let a = 666;
        println!("inside, a = {}", a); // 666
    }

    println!("outside, a = {}", a); // 1234
}