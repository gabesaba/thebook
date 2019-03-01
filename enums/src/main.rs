use std::time::Instant;
use std::u32;

fn main() {
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