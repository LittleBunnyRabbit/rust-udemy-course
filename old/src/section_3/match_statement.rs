pub fn match_statement() {
    let country_code = -1;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "invalid"
    };

    println!("{} => {}", country_code, country);
}