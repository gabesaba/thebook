use std::mem::drop;

pub fn run() {
    let pointer1 = MySmartPointer {
        data: String::from("Hello"),
    };
    println!("My smart pointer contains data '{}'", pointer1.data);
    let pointer2 = MySmartPointer {
        data: String::from("World"),
    };
    println!("My other smart pointer contains data '{}'", pointer2.data);

    println!("About to drop pointer2");
    drop(pointer2);
    println!("Dropped pointer2");

    println!("End of function. Cleanup commencing")
}

struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping data '{}'", self.data);
    }
}
