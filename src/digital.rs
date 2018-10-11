//! Digital I/O

use embedded_hal::digital::OutputPin as OldOutputPin;

/// Single digital push-pull output pin
/// (Fallible version. This will become the default after the next release)
///
/// *This trait is available if embedded-hal is built with the `"use-fallible-digital-traits"` feature.*
pub trait OutputPin {
    /// Error type
    type Error;

    /// Drives the pin low
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn set_low(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn set_high(&mut self) -> Result<(), Self::Error>;
}

/// Implementation of new OutputPin trait for previous trait
impl OutputPin for OldOutputPin
{
    type Error = ();

    /// Toggle pin output
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.set_low())
    }

     fn set_high(&mut self) -> Result<(), Self::Error> {
         Ok(self.set_high())
     }
}

/// Implementation of old OutputPin for new trait
/// TODO: Traits have been split and I can't see how to compose OutputPin + InputPin in this module..?
impl OldOutputPin for OutputPin<Error=()> {
    fn set_low(&mut self) {
        self.set_low().unwrap()
    }

    fn set_high(&mut self) {
        self.set_high().unwrap()
    }

    fn is_low(&self) -> bool {
        true
    }
    fn is_high(&self) -> bool {
        true
    }
}

/// Push-pull output pin that can read its output state
/// (Fallible version. This will become the default after the next release)
///
/// *This trait is available if embedded-hal is built with the `"unproven"` and
/// `"use-fallible-digital-traits"` features.*
pub trait StatefulOutputPin : OutputPin {
    /// Is the pin in drive high mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_high(&self) -> Result<bool, Self::Error>;

    /// Is the pin in drive low mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_low(&self) -> Result<bool, Self::Error>;
}

/// Output pin that can be toggled
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
pub trait ToggleableOutputPin {
    /// Error type
    type Error;

    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Single digital input pin
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
pub trait InputPin {
    /// Error type
    type Error;

    /// Is the input pin high?
    fn is_high(&self) -> Result<bool, Self::Error>;

    /// Is the input pin low?
    fn is_low(&self) -> Result<bool, Self::Error>;
}
