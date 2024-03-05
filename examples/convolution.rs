use gnuplot::{AxesCommon, Figure, PlotOption::Color};
use signal::core::{
    generators::{self, BufferWriter},
    signal::Signal,
};

const SIGNAL_LENGTH: usize = 80;
const SAMPLE_RATE: usize = 80;

fn main() {
    let mut sine_generator = generators::Sine::new(1.0, SAMPLE_RATE);
    let mut buffer = vec![0.0; SIGNAL_LENGTH];
    sine_generator.write_buffer(buffer.as_mut_slice());
    let sine_signal = Signal::from_buffer(buffer);

    let mut impulse_generator = generators::Impulse::new();
    let mut buffer = vec![0.0; 30];
    impulse_generator.write_buffer(buffer.as_mut_slice());
    let impulse_signal = Signal::from_buffer(buffer);
    let delay = impulse_signal.add_delay(10);

    let convolution = sine_signal * delay;

    let idx: Vec<usize> = (0..convolution.as_slice().len()).collect();
    let mut fg = Figure::new();
    fg.set_title("Scope plot");
    let axes = fg.axes2d();
    axes.lines(&idx, convolution.as_slice(), &[Color("red")]);
    axes.set_x_label("Time in ms", &[]);
    fg.show().unwrap();
}
