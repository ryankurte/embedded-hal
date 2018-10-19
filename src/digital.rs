//! Digital I/O

extern crate embedded_hal as hal_v03;

/// Single digital push-pull output pin
pub trait OutputPin {
    /// Drives the pin low
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn set_low(&mut self);

    /// Drives the pin high
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn set_high(&mut self);
}

/// Implementation of v0.3 fallible OutputPin for v0.2 traits
impl hal_v03::digital::OutputPin for OutputPin
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

/// Implementation of v0.2 OutputPin trait for v0.3 fallible output pins
impl OutputPin for hal_v03::digital::OutputPin<Error=()> {
    fn set_low(&mut self) {
        self.set_low().unwrap()
    }

    fn set_high(&mut self) {
        self.set_high().unwrap()
    }
}


/// Push-pull output pin that can read its output state
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait StatefulOutputPin {
    /// Is the pin in drive high mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_high(&self) -> bool;

    /// Is the pin in drive low mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_low(&self) -> bool;
}

/// Implementation of v0.3 fallible StatefulOutputPin for v0.2 traits
#[cfg(feature = "not-sure-how")]
impl hal_v03::digital::StatefulOutputPin for StatefulOutputPin
{
    type Error = ();

    /// Toggle pin output
    fn is_set_low(&self) -> Result<(), Self::Error> {
        Ok(self.is_set_low())
    }

     fn is_set_high(&self) -> Result<(), Self::Error> {
         Ok(self.is_set_high())
     }
}


/// Implementation of v0.2 StatefulOutputPin trait for v0.3 fallible pins
#[cfg(feature = "unproven")]
impl StatefulOutputPin for hal_v03::digital::StatefulOutputPin<Error=()> {
    fn is_set_low(&self) -> bool {
        self.is_set_low().unwrap()
    }

    fn is_set_high(&self) -> bool {
        self.is_set_high().unwrap()
    }
}

/// Output pin that can be toggled
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
#[cfg(feature = "unproven")]
pub trait ToggleableOutputPin {
    /// Toggle pin output.
    fn toggle(&mut self);
}

/// If you can read **and** write the output state, a pin is
/// toggleable by software.
///
/// ```
/// use embedded_hal::digital::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
/// use embedded_hal::digital::toggleable;
///
/// /// A virtual output pin that exists purely in software
/// struct MyPin {
///     state: bool
/// }
///
/// impl OutputPin for MyPin {
///    fn set_low(&mut self) {
///        self.state = false;
///    }
///    fn set_high(&mut self) {
///        self.state = true;
///    }
/// }
///
/// impl StatefulOutputPin for MyPin {
///    fn is_set_low(&self) -> bool {
///        !self.state
///    }
///    fn is_set_high(&self) -> bool {
///        self.state
///    }
/// }
///
/// /// Opt-in to the software implementation.
/// impl toggleable::Default for MyPin {}
///
/// let mut pin = MyPin { state: false };
/// pin.toggle();
/// assert!(pin.is_set_high());
/// pin.toggle();
/// assert!(pin.is_set_low());
/// ```
#[cfg(feature = "unproven")]
pub mod toggleable {
    use super::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// Software-driven `toggle()` implementation.
    ///
    /// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
    pub trait Default: OutputPin + StatefulOutputPin {}

    impl<P> ToggleableOutputPin for P
    where
        P: Default,
    {
        /// Toggle pin output
        fn toggle(&mut self) {
            if self.is_set_low() {
                self.set_high();
            } else {
                self.set_low();
            }
        }
    }
}

/// Single digital input pin
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Is the input pin high?
    fn is_high(&self) -> bool;

    /// Is the input pin low?
    fn is_low(&self) -> bool;
}

/// Implementation of v0.3 fallible InputPin for v0.2 traits
#[cfg(feature = "unproven")]
impl hal_v03::digital::InputPin for InputPin
{
    type Error = ();

    /// Toggle pin output
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }

     fn is_high(&self) -> Result<bool, Self::Error> {
         Ok(self.is_high())
     }
}


/// Implementation of v0.2 InputPin trait for v0.3 fallible pins
#[cfg(feature = "unproven")]
impl InputPin for hal_v03::digital::InputPin<Error=()> {
    fn is_low(&self) -> bool {
        self.is_low().unwrap()
    }

    fn is_high(&self) -> bool {
        self.is_high().unwrap()
    }
}
