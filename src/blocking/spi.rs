//! Blocking SPI API

/// Blocking transfer
pub trait Transfer<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave. Returns the `words` received from the slave
    fn transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `words` to the slave, ignoring all the incoming words
    fn write(&mut self, words: &[W]) -> Result<(), Self::Error>;
}

/// Blocking transfer
pub mod transfer {
    /// Default implementation of `blocking::spi::Transfer<W>` for implementers of
    /// `spi::FullDuplex<W>`
    pub trait Default<W>: ::spi::FullDuplex<W> {}

    impl<W, S> ::blocking::spi::Transfer<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn transfer<'w>(&mut self, words: &'w mut [W]) -> Result<&'w [W], S::Error> {
            for word in words.iter_mut() {
                block!(self.send(word.clone()))?;
                *word = block!(self.read())?;
            }

            Ok(words)
        }
    }
}

/// Blocking write
pub mod write {
    /// Default implementation of `blocking::spi::Write<W>` for implementers of `spi::FullDuplex<W>`
    pub trait Default<W>: ::spi::FullDuplex<W> {}

    impl<W, S> ::blocking::spi::Write<W> for S
    where
        S: Default<W>,
        W: Clone,
    {
        type Error = S::Error;

        fn write(&mut self, words: &[W]) -> Result<(), S::Error> {
            for word in words {
                block!(self.send(word.clone()))?;
                block!(self.read())?;
            }

            Ok(())
        }
    }
}


struct Transaction<'a, S: 'a, E> {
    spi: &'a S,
    error: Option<E>,
}

pub trait Transact<W> {
    type Error;

    fn read(self, &mut [W]) -> Self;
    fn write(self, &[W]) -> Self;
    fn end(self) -> Result<(), Self::Error>;
}

impl <'a, S, E, W>Transact<W> for Transaction<'a, S, E> 
where
    S: ::spi::FullDuplex<W>,
    W: Clone,
{
    type Error = S::Error;

    fn write(self, data: &[W]) -> Self {
        if let Some(e) = self.error {
            return self;
        }

        // write from data

        self
    }
    fn read(self, data: &mut [W]) -> Self {
        if let Some(e) = self.error {
            return self;
        }

        // read into data

        self
    }

    fn end(self) -> Result<(), Self::Error> {
        // de-assert

        match self.error {
            Some(e) => Err(e),
            None => Ok(())
        }
    }
}