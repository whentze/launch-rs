extern crate portmidi as pm;

mod color;
mod launchpad;
mod mk2;
mod mini;

pub use color::*;
pub use launchpad::*;
pub use mk2::*;
pub use mini::*;

pub type Error = Box<std::error::Error + Send + Sync>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
