#![allow(unused, unused_imports)]
use hello_macro::HelloMacro;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize as yo;
use whoami::WhoAmI;
use hello_macro::adx;

#[derive(WhoAmI)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(HelloMacro, yo)]
struct Person;

#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes1111!");
//     }
// }

fn main() {
    let p = Point { x: 0.0, y: 0.0 };
    println!("x = {}, y = {}", p.x, p.y);
    Person::hello_macro();
    Pancakes::hello_macro();

    println!("end:{}", 345);
    adx();
}

#[test]
fn test1() {
    smol::block_on(async {
        println!("123456");
    });

    println!("test..");
}

struct Person111 {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// This is what #[derive(Serialize)] would generate.
impl yo for Person111 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("age", &self.age)?;
        s.serialize_field("phones", &self.phones)?;
        s.end()
    }
}
