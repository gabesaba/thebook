use std::time::Instant;
use std::u32;

fn main() {
    let maybe_here = Option::Some(String::from("419"));
    let also_maybe_here = Option::None;
    print_option(&maybe_here);
    print_option(&also_maybe_here);

    let start = Instant::now();

    for i in 0..u32::MAX {
        let ip = IpAddr::V4(i);

        if i % 10000000 == 0 {
            println!("{}", ip.stringify());
            let elapsed = start.elapsed().as_secs();
            if elapsed > 0 {
                println!("Currently processing at rate of {}/sec", (i as u64) / elapsed);
            }
        }
    }
}

fn print_option(potential: &Option<String>) {
    match potential {
        Option::Some(s) => println!("Echo {}", s),
        Option::None => println!("Nothing but dust and echoes.. ()")
    }
}

enum IpAddr {
    V4(u32),
    V6(u64)
}

impl IpAddr {
    fn stringify(&self) -> String {
        match self {
            IpAddr::V4(x) => {
                return format!("{}.{}.{}.{}",
                               (x >> 24) & 255_u32,
                               (x >> 16) & 255_u32,
                               (x >> 8) & 255_u32,
                               (x >> 0) & 255_u32);
            },
            IpAddr::V6(x) => {
                return String::from("Not yet supported..");
            },
        }
    }
}