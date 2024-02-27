/// Module containing the `Signal` struct.
use std::ops::Add;

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
    pub fn data(&self) -> &Vec<T> {
        &self.data
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
    /// assert_eq!(c.data().as_slice(), &[5, 7, 3]);
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
