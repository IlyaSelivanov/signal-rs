use gnuplot::{AxesCommon, Color, Figure};
use signal::core::generators::{self, BufferWriter};

const SIGNAL_LENGTH: usize = 512;
const SAMPLE_RATE: usize = 512;

fn main() {
    let mut signal = generators::Sine::new(1.0, SAMPLE_RATE);
    // let mut signal = generator::Impulse::new();
    let mut buffer = vec![0.0; SIGNAL_LENGTH];

    signal.write_buffer(buffer.as_mut_slice());

    let idx: Vec<usize> = (0..buffer.len()).collect();
    let mut fg = Figure::new();
    fg.set_title("Scope plot");
    let axes = fg.axes2d();
    axes.lines(&idx, buffer, &[Color("red")]);
    axes.set_x_label("Time in ms", &[]);
    fg.show().unwrap();
}
