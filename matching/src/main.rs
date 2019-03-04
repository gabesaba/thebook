fn main() {
    let c = Drink::Coke;
    let g = Drink::Gatorade(Color::Red);
    let p = Drink::Perrier;
    let drinks = [c,g,p];
    for drink in drinks.iter() {
        println!("{:?}: {}", drink, judge_drink(drink));
        println!("Color: {}", get_color(drink));
    }

    let a = Option::Some(5);
    let b = plus_one(a);
    let c = Option::None;
    let d = plus_one(c);


}


// Single arm of match
fn get_color(drink: &Drink) -> String {
    if let Drink::Gatorade(x) = drink {
        format!("{:?}", x)
    } else {
        String::from("No color")
    }
}

// From Book
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn judge_drink(drink: &Drink) -> &'static str{
    match drink {
        Drink::Coke => "Warren Buffet approves",
        Drink::Gatorade(color) => "Electrolytes!",
        Drink::Perrier => "Mmm, Fancy"
    }
}

#[derive(Debug)]
enum Drink {
    Gatorade(Color),
    Coke,
    Perrier
}

#[derive(Debug)]
enum Color {
    Red,
    Orange,
    Blue
}