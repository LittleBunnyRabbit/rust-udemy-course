pub fn if_statement() {
    let temp = 32;

    if temp > 30 {
        println!("So hot!");
    } else if temp < 10 {
        println!("reall cold!");
    } else {
        println!("Its kinda nice");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("{}", day);

    println!("it is {}",
        if temp > 20 { "hot" } 
        else if temp < 10 { "cold" }
        else { "ok" }
    );

    println!("it is {}",
        if temp > 20 { 
            if temp > 30 { "very hot" }
            else { "hot" }
        } 
        else if temp < 10 { "cold" }
        else { "ok" }
    );

    println!("it is {}", if temp > 20 { if temp > 30 { "very hot" } else { "hot" } } else if temp < 10 { "cold" } else { "ok" });
}