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
    fn guess() -> Result<Self, Error> {
        let midi = pm::PortMidi::new().expect("Failed to open PortMidi Instance!");
        let devs = midi.devices().expect("Failed to get Midi Device!");
        let mut input : Option<i32> = None;
        let mut output : Option<i32> = None;

        for d in devs {
            if d.name().contains("Launchpad") {
                if d.is_output() {
                    output = Some(d.id() as i32);
                } else if (d.is_input()) {
                    input = Some(d.id() as i32);
                }
                if input.is_some() && output.is_some() {
                    break;
                }
            }
        }
        let input_port = input.expect("No Launchpad found");
        let output_port = output.expect("No Launchpad found");
        println!("{}\t{}", output_port, input_port);

        let input_device = midi.device(input_port).expect("no input");
        let output_device = midi.device(output_port).expect("no output");

        let lp_input = midi.input_port(input_device, 1024).expect("failed to open input port");
        let lp_output = midi.output_port(output_device, 1024).expect("failed to open output port");

        Ok(LaunchpadMini{
            input_port: lp_input,
            output_port: lp_output,
            midi: Some(midi),
        })
    }
    fn light_all(&mut self, color: Self::Color) {
        unimplemented!();
    }
}
