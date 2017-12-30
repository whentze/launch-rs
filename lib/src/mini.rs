use pm;
use Error;
use color::RGColor;
use launchpad::Launchpad;

/// A Launchpad Mini Device. This only has red and green LEDs
pub struct LaunchpadMini {
    input_port: pm::InputPort,
    output_port: pm::OutputPort,
    midi: Option<pm::PortMidi>,
}

impl Launchpad for LaunchpadMini {
    type Color = RGColor;
    type PaletteIndex = u8;
    fn guess() -> Result<Self, Error> {
        unimplemented!();
    }
    fn light_all(&mut self, color: Self::PaletteIndex) {
        unimplemented!();
    }
}
