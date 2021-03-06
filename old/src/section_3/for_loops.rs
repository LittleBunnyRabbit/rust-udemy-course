pub fn for_loops() {
    for x in 1..11 { // 1 - 10
        if x == 3 { continue }
        if x == 8 { break }

        println!("x = {}", x);
    }

    for (position, y) in (30..41).enumerate() {
        println!("position = {}, y = {}", position, y);
    }
}