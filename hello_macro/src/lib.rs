#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(feature = "hp")]
extern crate whoami;

#[cfg(feature = "hp")]
pub use whoami::HelloMacro;
