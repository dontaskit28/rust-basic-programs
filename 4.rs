fn main() {
    let sky = "cloudy"; // can be "cloudy", "sunny", "rainy"
    let temperature = "warm"; // can be "warm", "cold", "freezing"

    match (sky,temperature) {
        ("cloudy","warm") => println!("It's cloudy but it's warm"),
        ("cloudy","cold") => println!("Plesant weather"),
        ("cloudy","freezing") => println!("It's freezing outside"),
        ("sunny","warm") => println!("It's hot sunny day"),
        ("sunny","cold") => println!("It's a cold sunny day"),
        ("rainy","warm") => println!("It's raining"),
        ("rainy","cold") => println!("It's raining and it's cold"),
        ("rainy","freezing") => println!("It's raining and it's freezing"),
        _ => println!("Not sure what to do"),
    };
}