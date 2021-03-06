#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::stdin;

pub fn if_statement() {
    let temp = 35;

    if temp > 30 {
        println!("Really hot outside!");
    } else if temp < 10 {
        println!("Really cold outside!");
    } else {
        println!("Temperature is OK");
    }

    let day = if temp > 30 { "sunny" } else { "cloudy" };

    println!("Today is {}", day);

    println!("It is {}", 
        if temp > 20 { "hot" } 
        else if temp < 10 { "cold" } 
        else { "OK" }
    );

    println!("It is {}", 
        if temp > 20 {
            if temp > 30 { "very hot"}
            else { "hot" }
        }
        else if temp < 10 { "cold" }
        else { "OK" }
    )
}

pub fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop { // while true
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break; }
    }
}

pub fn for_loops() {
    for x in 1..11 {
        if x == 3 { continue; }
        if x == 8 { break; }
        println!("x = {}", x);
    }

    for (position, y) in (30..41).enumerate() {
        println!("{}: {}", position, y);
    }
}

pub fn match_statement() {
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=100 => "unknown", // 1 - 1000
        _ => "invalid" // everything else
    };

    println!("The country with code {} is {}", country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };
}

// Example
enum State {
    Locked, 
    Failed,
    Unlocked
}

pub fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => entry.push_str(&input.trim_end()),
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) { 
                    state = State::Failed; 
                }
            }
            State::Failed => {
                println!("Failed!");
                entry.clear(); // entry = ""
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Unlocked!");
                return;
            }
        }
    }
}