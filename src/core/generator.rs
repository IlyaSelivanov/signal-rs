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
///
/// let mut signal = Impulse::new();
/// let mut buffer = vec![0.0; 3];
///
/// signal.write_buffer(buffer.as_mut_slice());
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

/// A struct representing a sine wave generator.
///
/// # Example
///
/// ```
/// use signal::core::generator::Sine;
///
/// let mut signal = Sine::new(1.0, 512);
/// let mut buffer = vec![0.0; 512];
///
/// signal.write_buffer(buffer.as_mut_slice());
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
