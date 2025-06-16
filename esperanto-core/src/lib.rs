// Main entry point for the Esperanto Core library
// Declares public modufles so esperanto-server can use them


pub mod policy;
pub mod verifier;
pub mod attesation;
pub mod error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
