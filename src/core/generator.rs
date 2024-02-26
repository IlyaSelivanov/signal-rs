/// A trait for writing to a buffer.
pub trait BufferWriter: Iterator {
    /// Writes the next `n` samples to the buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to write to.
    ///
    /// # Example
    ///
    /// ```
    /// use signal::core::generator::BufferWriter;
    ///
    /// let mut signal = vec![1.0; 3];
    /// let mut buffer = vec![0.0; 3];
    ///
    /// signal.write_buffer(buffer.as_mut_slice());
    /// assert_eq!(buffer, vec![1.0, 1.0, 1.0]);    
    /// ```
    fn write_buffer(&mut self, buffer: &mut [<Self as Iterator>::Item]) {
        for e in buffer.iter_mut() {
            *e = self.next().unwrap()
        }
    }
}

/// A struct representing an impulse generator.
///
/// # Example
///
/// ```
/// use signal::core::generator::Impulse;
/// use signal::core::generator::BufferWriter;
///
/// let mut signal = Impulse::new();
/// let mut buffer = vec![0.0; 3];
/// signal.write_buffer(buffer.as_mut_slice());
///
/// assert_eq!(buffer, vec![1.0, 0.0, 0.0]);
/// ```
pub struct Impulse {
    impulse_sent: bool,
}

impl Impulse {
    /// Creates a new `Impulse` generator.
    /// # Returns
    ///
    /// A new `Impulse` generator.
    pub fn new() -> Impulse {
        Impulse {
            impulse_sent: false,
        }
    }
}

impl Default for Impulse {
    /// Creates a new `Impulse` generator with default values.
    fn default() -> Self {
        Self::new()
    }
}

impl BufferWriter for Impulse {}

impl Iterator for Impulse {
    type Item = f32;

    /// Generates the next sample of the impulse.
    ///
    /// # Returns
    ///
    /// The next sample of the impulse as an `Option<f32>`.
    fn next(&mut self) -> Option<f32> {
        if self.impulse_sent {
            Some(0.0)
        } else {
            self.impulse_sent = true;
            Some(1.0)
        }
    }
}

/// A struct representing a step generator.
///
/// # Example
///
/// ```
/// use signal::core::generator::Step;
/// use signal::core::generator::BufferWriter;
///
/// let mut signal = Step::new(1);
/// let mut buffer = vec![0.0; 3];
/// signal.write_buffer(buffer.as_mut_slice());
///
/// assert_eq!(buffer, vec![0.0, 1.0, 1.0]);
/// ```
pub struct Step {
    step_pos: usize,
}

impl Step {
    /// Creates a new `Step` generator with the specified step position.
    ///
    /// # Arguments
    ///
    /// * `step_pos` - The position of the step.
    ///
    /// # Returns
    ///
    /// A new `Step` generator.
    pub fn new(step_pos: usize) -> Step {
        Step { step_pos }
    }
}

impl BufferWriter for Step {}

impl Iterator for Step {
    type Item = f32;

    /// Generates the next sample of the step.
    ///
    /// # Returns
    ///
    /// The next sample of the step as an `Option<f32>`.
    fn next(&mut self) -> Option<f32> {
        let sample = if self.step_pos > 0 {
            self.step_pos -= 1;
            0.0
        } else {
            1.0
        };

        Some(sample)
    }
}

/// A struct representing a sine wave generator.
///
/// # Example
///
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use signal::core::generator::Sine;
/// use signal::core::generator::BufferWriter;
///
/// let mut signal = Sine::new(2.0, 8);
/// let mut buffer = vec![0.0; 10];
/// signal.write_buffer(buffer.as_mut_slice());
///
/// assert_approx_eq!(buffer[0], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[2], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], -1.0, 1e-5f32);
/// ```
pub struct Sine {
    step_pos: usize,
    freq: f32,
    sample_rate: usize,
}

impl Sine {
    /// Creates a new `Sine` generator with the specified frequency and sample rate.
    ///
    /// # Arguments
    ///
    /// * `freq` - The frequency of the sine wave.
    /// * `sample_rate` - The sample rate of the generator.
    ///
    /// # Returns
    ///
    /// A new `Sine` generator.
    pub fn new(freq: f32, sample_rate: usize) -> Sine {
        Sine {
            step_pos: 0,
            freq,
            sample_rate,
        }
    }
}

impl BufferWriter for Sine {}

impl Iterator for Sine {
    type Item = f32;

    /// Generates the next sample of the sine wave.
    ///
    /// # Returns
    ///
    /// The next sample of the sine wave as an `Option<f32>`.
    fn next(&mut self) -> Option<f32> {
        let t = self.step_pos as f32 / self.sample_rate as f32;
        self.step_pos += 1;
        Some((t * self.freq * 2.0 * std::f32::consts::PI).sin())
    }
}

/// A struct representing a square wave generator.
///
/// # Example
///
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use signal::core::generator::Sawtooth;
/// use signal::core::generator::BufferWriter;
///
/// let mut signal = Sawtooth::new(4.0, 16);
/// let mut buffer = vec![0.0; 16];
/// signal.write_buffer(buffer.as_mut_slice());
///
/// assert_approx_eq!(buffer[0], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], 0.25, 1e-5f32);
/// assert_approx_eq!(buffer[2], 0.5, 1e-5f32);
/// assert_approx_eq!(buffer[3], 0.75, 1e-5f32);
/// assert_approx_eq!(buffer[4], 0.0, 1e-5f32);
/// ```
pub struct Sawtooth {
    step_pos: usize,
    freq: f32,
    sample_rate: usize,
}

impl Sawtooth {
    /// Creates a new `Sawtooth` generator with the specified frequency and sample rate.
    ///
    /// # Arguments
    ///
    /// * `freq` - The frequency of the sawtooth wave.
    /// * `sample_rate` - The sample rate of the generator.
    ///
    /// # Returns
    ///
    /// A new `Sawtooth` generator.
    pub fn new(freq: f32, sample_rate: usize) -> Sawtooth {
        Sawtooth {
            step_pos: 0,
            freq,
            sample_rate,
        }
    }
}

impl BufferWriter for Sawtooth {}

impl Iterator for Sawtooth {
    type Item = f32;

    /// Generates the next sample of the sawtooth wave.
    ///
    /// # Returns
    ///
    /// The next sample of the sawtooth wave as an `Option<f32>`.
    fn next(&mut self) -> Option<f32> {
        let t = self.step_pos as f32 / self.sample_rate as f32;

        self.step_pos += 1;
        if self.step_pos >= self.sample_rate {
            self.step_pos = 0;
        }

        Some((t * self.freq).fract())
    }
}

/// A struct representing a square wave generator.
///
/// # Example
///
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use signal::core::generator::Square;
/// use signal::core::generator::BufferWriter;
///
/// let mut signal = Square::new(2.0, 8);
/// let mut buffer = vec![0.0; 10];
/// signal.write_buffer(buffer.as_mut_slice());
///
/// assert_approx_eq!(buffer[0], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[2], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], 1.0, 1e-5f32);
/// ```
pub struct Square {
    step_pos: usize,
    freq: f32,
    sample_rate: usize,
}

impl Square {
    /// Creates a new `Square` generator with the specified frequency and sample rate.
    ///
    /// # Arguments
    ///
    /// * `freq` - The frequency of the square wave.
    /// * `sample_rate` - The sample rate of the generator.
    ///
    /// # Returns
    ///
    /// A new `Square` generator.
    pub fn new(freq: f32, sample_rate: usize) -> Square {
        Square {
            step_pos: 0,
            freq,
            sample_rate,
        }
    }
}

impl BufferWriter for Square {}

impl Iterator for Square {
    type Item = f32;

    /// Generates the next sample of the square wave.
    ///
    /// # Returns
    ///
    /// The next sample of the square wave as an `Option<f32>`.
    fn next(&mut self) -> Option<f32> {
        let t = self.step_pos as f32 / self.sample_rate as f32;

        self.step_pos += 1;
        if self.step_pos >= self.sample_rate {
            self.step_pos = 0;
        }

        if (t * self.freq).fract() < 0.5 {
            Some(0.0)
        } else {
            Some(1.0)
        }
    }
}
