//! Blocking SPI API

/// Blocking transfer
pub trait Transfer<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave. Returns the `words` received from the slave
    fn try_transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn try_write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = W>;
}

/// ManagedCS marker trait specifies that all `spi` operations will be preceded by
/// asserting the CS pin, and followed by de-asserting the CS pin.
///
/// TODO: document wrappers that can be used where this is required
pub trait ManagedCs {}

/// Blocking transfer
pub mod transfer {
    /// Default implementation of `blocking::spi::Transfer<W>` for implementers of
    /// `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Transfer<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], S::Error> {
            for word in words.iter_mut() {
                block!(self.try_send(word.clone()))?;
                *word = block!(self.try_read())?;
            }

            Ok(words)
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers of `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::Write<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write(&mut self, words: &[W]) -> Result<(), S::Error> {
            for word in words {
                block!(self.try_send(word.clone()))?;
                block!(self.try_read())?;
            }

            Ok(())
        }
    }
}

/// Blocking write (iterator version)
pub mod write_iter {
    /// Default implementation of `blocking::spi::WriteIter<W>` for implementers of
    /// `spi::FullDuplex<W>`
    pub trait Default<W>: crate::spi::FullDuplex<W> {}

    impl<W, S> crate::blocking::spi::WriteIter<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), S::Error>
        where
            WI: IntoIterator<Item = W>,
        {
            for word in words.into_iter() {
                block!(self.try_send(word.clone()))?;
                block!(self.try_read())?;
            }

            Ok(())
        }
    }
}

/// Provides SpiWithCS wrapper around an spi::* and OutputPin impl
pub mod spi_with_cs {

    use core::fmt::Debug;
    use core::marker::PhantomData;

    use super::*;
    use crate::digital::OutputPin;

    /// SpiWithCS wraps an blocking::spi* implementation with Chip Select (CS)
    /// pin management.
    /// For sharing SPI between peripherals, see [shared-bus]()
    pub struct SpiWithCs<Spi, SpiError, Pin, PinError> {
        spi: Spi,
        cs: Pin,

        _spi_err: PhantomData<SpiError>,
        _pin_err: PhantomData<PinError>,
    }

    /// SpiWithCsErr provies an error numberation over generic Spi and Pin variants
    #[derive(Clone, Debug, PartialEq)]
    pub enum SpiWithCsErr<SpiError, PinError> {
        /// Underlying SPI error
        Spi(SpiError),
        /// Underlying Pin error
        Pin(PinError),
    }

    /// ManagedCS marker trait indicates Chip Select management is automatic
    impl<Spi, SpiError, Pin, PinError> ManagedCs for SpiWithCs<Spi, SpiError, Pin, PinError> {}

    impl<Spi, SpiError, Pin, PinError> SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Pin: crate::digital::OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        /// Create a new SpiWithCS wrapper with the provided Spi and Pin
        pub fn new(spi: Spi, cs: Pin) -> Self {
            Self {
                spi,
                cs,
                _spi_err: PhantomData,
                _pin_err: PhantomData,
            }
        }

        /// Fetch references to the inner Spi and Pin types.
        /// Note that using these directly will violate the `ManagedCs` constraint.
        pub fn inner(&mut self) -> (&mut Spi, &mut Pin) {
            (&mut self.spi, &mut self.cs)
        }
    }

    impl<Spi, SpiError, Pin, PinError> Transfer<u8> for SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Spi: Transfer<u8, Error = SpiError>,
        Pin: OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        type Error = SpiWithCsErr<SpiError, PinError>;

        /// Attempt an SPI transfer with automated CS assert/deassert
        fn try_transfer<'w>(&mut self, data: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
            // First assert CS
            self.cs.try_set_low().map_err(SpiWithCsErr::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_res = self.spi.try_transfer(data).map_err(SpiWithCsErr::Spi);

            // Deassert CS
            self.cs.try_set_high().map_err(SpiWithCsErr::Pin)?;

            // Return failures
            spi_res
        }
    }

    impl<Spi, SpiError, Pin, PinError> Write<u8> for SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Spi: Write<u8, Error = SpiError>,
        Pin: OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        type Error = SpiWithCsErr<SpiError, PinError>;

        /// Attempt an SPI write with automated CS assert/deassert
        fn try_write<'w>(&mut self, data: &'w [u8]) -> Result<(), Self::Error> {
            // First assert CS
            self.cs.try_set_low().map_err(SpiWithCsErr::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_res = self.spi.try_write(data).map_err(SpiWithCsErr::Spi);

            // Deassert CS
            self.cs.try_set_high().map_err(SpiWithCsErr::Pin)?;

            // Return failures
            spi_res
        }
    }

    impl<Spi, SpiError, Pin, PinError> WriteIter<u8> for SpiWithCs<Spi, SpiError, Pin, PinError>
    where
        Spi: WriteIter<u8, Error = SpiError>,
        Pin: OutputPin<Error = PinError>,
        SpiError: Debug,
        PinError: Debug,
    {
        type Error = SpiWithCsErr<SpiError, PinError>;

        /// Attempt an SPI write_iter with automated CS assert/deassert
        fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            // First assert CS
            self.cs.try_set_low().map_err(SpiWithCsErr::Pin)?;

            // Attempt the transfer, storing the result for later
            let spi_res = self.spi.try_write_iter(words).map_err(SpiWithCsErr::Spi);

            // Deassert CS
            self.cs.try_set_high().map_err(SpiWithCsErr::Pin)?;

            // Return failures
            spi_res
        }
    }
}
