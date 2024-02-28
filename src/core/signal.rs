/// Module containing the `Signal` struct.
use std::ops::{Add, Mul};

/// A struct representing a signal.
pub struct Signal<T> {
    data: Vec<T>,
}

impl<T> Signal<T> {
    /// Creates a new `Signal` with the given data.
    ///
    /// * `data` - The data to create the signal with.
    ///
    /// # Returns
    ///
    /// A new `Signal` with the given data.
    pub fn from_buffer(data: Vec<T>) -> Signal<T> {
        Signal { data }
    }

    /// Returns the data of the signal.
    ///
    /// # Returns
    ///
    /// The data of the signal.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Multiplies the signal by a scalar.
    ///
    /// * `scalar` - The scalar to multiply the signal by.
    ///
    /// # Returns
    ///
    /// The signal multiplied by the scalar.
    ///
    /// # Example
    ///
    /// ```
    /// use signal::core::signal::Signal;
    ///
    /// let a = Signal::from_buffer(vec![1, 2, 3]);
    /// let b = a.mull_by_scalar(2);
    ///
    /// assert_eq!(b.as_slice(), &[2, 4, 6]);
    /// ```
    pub fn mull_by_scalar(&self, scalar: T) -> Signal<T>
    where
        T: std::ops::Mul<Output = T> + Copy,
    {
        let buffer = self.data.iter().map(|x| *x * scalar).collect::<Vec<T>>();
        Signal::from_buffer(buffer)
    }

    /// Adds a delay to the signal.
    ///
    /// * `delay` - The delay to add to the signal.
    ///
    /// # Returns
    ///
    /// The signal with the added delay.
    ///
    /// # Example
    ///
    /// ```
    /// use signal::core::signal::Signal;
    ///
    /// let a = Signal::from_buffer(vec![1, 2, 3, 4, 5]);
    /// let b = a.add_delay(2);
    ///
    /// assert_eq!(b.as_slice(), &[0, 0, 1, 2, 3]);
    /// ```
    pub fn add_delay(&self, delay: usize) -> Signal<T>
    where
        T: Default + Copy,
    {
        let buffer = (0..delay)
            .map(|_| Default::default())
            .chain(
                self.data
                    .iter()
                    .cloned()
                    .take(self.data.len() - delay)
                    .collect::<Vec<T>>(),
            )
            .collect::<Vec<T>>();
        Signal::from_buffer(buffer)
    }
}

impl<T> Add for Signal<T>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Self;

    /// Adds two signals together.
    ///
    /// * `rhs` - The right hand side of the addition.
    ///
    /// # Returns
    ///
    /// The sum of the two signals.
    ///
    /// # Example
    ///
    /// ```
    /// use signal::core::signal::Signal;
    ///
    /// let a = Signal::from_buffer(vec![1, 2, 3]);
    /// let b = Signal::from_buffer(vec![4, 5]);
    ///
    /// let c = a + b;
    ///
    /// assert_eq!(c.as_slice(), &[5, 7, 3]);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let len = usize::max(self.data.len(), rhs.data.len());
        let buffer = (0..len)
            .map(|i| {
                let a = if i < self.data.len() {
                    self.data[i]
                } else {
                    Default::default()
                };
                let b = if i < rhs.data.len() {
                    rhs.data[i]
                } else {
                    Default::default()
                };
                a + b
            })
            .collect::<Vec<T>>();

        Signal::from_buffer(buffer.clone())
    }
}

impl<T> Mul for Signal<T>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy,
{
    type Output = Self;

    /// Multiplies two signals together. This is done using the convolution theorem.
    ///
    /// * `rhs` - The right hand side of the multiplication.
    ///
    /// # Returns
    ///
    /// The product of the two signals.
    ///
    /// # Example
    ///
    /// ```
    /// use signal::core::signal::Signal;
    ///
    /// let a = Signal::from_buffer(vec![1, 2, 3]);
    /// let b = Signal::from_buffer(vec![4, 5]);
    ///
    /// let c = a * b;
    ///
    /// assert_eq!(c.as_slice(), &[4, 13, 22, 15]);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.data.len() + rhs.data.len() - 1;
        let mut buffer = vec![Default::default(); len];

        for i in 0..self.data.len() {
            for j in 0..rhs.data.len() {
                let a = self.data[i];
                let b = rhs.data[j];
                let c = a * b;
                buffer[i + j] = buffer[i + j] + c;
            }
        }

        Signal::from_buffer(buffer)
    }
}
