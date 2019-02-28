fn main() {
    let gabe = create_person("Gabe".to_string(), 52);

    println!("Debug print: {:?}", gabe);
    let ed = Person{name: String::from("Ed"), age: 33};
    let people = [&gabe, &ed, &create_bob()];
    for person in people.iter() {
        print_person(person);
    }

    let evil_gabe = Person{name: String::from("EvilGabe"), ..gabe}; // This is allowed because int is copyable
    print_person(&evil_gabe);
    print_person(&gabe);

    let p1 = Point(-2, 2);
    print_point(&p1);
}

// Shorthand: don't have to specify struct field names when param names are same
fn create_person(name: String, age: u16) -> Person {
    Person{name, age}
}

fn create_bob() -> Person {
    let name = "Bob".to_string();
    let age = 70;
    Person{name, age}
}

fn print_person(person: &Person) {
    println!("{}, {}", person.name, person.age);
}

fn print_point(point: &Point) {
    println!("x: {} y: {} norm: {}", point.0, point.1, point.norm());
}

#[derive(Debug)]
struct Person{
    name: String,
    age: u16,
}

struct Point(i16, i16);

impl Point {
    fn norm(&self) -> i16 {
        self.0.abs() * self.1.abs()
    }
}