//! Definition of Launchpad devices.
//!
//! For now, only Launchpad Mark 2 devices are supported.

use pm;
use Error;

pub trait Launchpad {
    type Color;
    type PaletteIndex;
    fn guess() -> Result<Self, Error> where Self : Sized;
    fn light_all(&mut self, color: Self::PaletteIndex);
}