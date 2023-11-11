use std::marker::PhantomData;

pub trait Sample: Copy {
    /// Minimum sample value.
    fn min() -> Self;
    /// Maximum sample value.
    fn max() -> Self;

    fn from_f64(n: f64) -> Self;

    fn as_f64(self) -> f64;
}

macro_rules! sample {
    ($(($num:ty, $min:expr, $max:expr)),*) => {$(
        impl Sample for $num {
            #[inline]
            fn min() -> Self {
                $min
            }

            #[inline]
            fn max() -> Self {
                $max
            }

            #[inline]
            fn from_f64(n: f64) -> Self {
                n as Self
            }

            #[inline]
            fn as_f64(self) -> f64 {
                self as f64
            }
        }
    )*}
}

sample! {
    (i16, std::i16::MIN, std::i16::MAX),
    (u16, std::u16::MIN, std::u16::MAX),
    (u8, std::u8::MIN, std::u8::MAX),
    (i8, std::i8::MIN, std::i8::MAX),
    (f32, -1.0, 1.0)
}

/// Audio device
pub trait Audio {
    type Sample: Sample;

    /// Return the samples per second of the device.
    fn sample_rate() -> u64;

    /// Returns true if the channel is single-channel.
    fn mono() -> bool;
}

/// 44100Hz, stereo.
pub struct Stereo44100<T>(pub PhantomData<T>);

/// 44100Hz, mono.
pub struct Mono44100<T>(PhantomData<T>);

impl<T: Sample> Audio for Stereo44100<T> {
    type Sample = T;

    #[inline]
    fn sample_rate() -> u64 {
        44100
    }

    #[inline]
    fn mono() -> bool {
        false
    }
}

impl<T: Sample> Audio for Mono44100<T> {
    type Sample = T;

    #[inline]
    fn sample_rate() -> u64 {
        44100
    }

    #[inline]
    fn mono() -> bool {
        false
    }
}

impl Audio for () {
    type Sample = u8;

    fn sample_rate() -> u64 {
        4
    }

    fn mono() -> bool {
        true
    }
}

/*
impl Sample for i16 {
    fn min() -> i16 {
        i16::MIN
    }

    fn max() -> i16 {
        i16::MAX
    }

    fn from_f64(n: f64) -> i16 {
        n as i16
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}



impl Sample for u16 {
    fn min() -> u16 {
        u16::MIN
    }

    fn max() -> u16 {
        u16::MAX
    }

    fn from_f64(n: f64) -> u16 {
        n as u16
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}


impl Sample for u8 {
    fn min() -> u8 {
        u8::MIN
    }

    fn max() -> u8 {
        u8::MAX
    }

    fn from_f64(n: f64) -> u8 {
        n as u8
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl Sample for i8 {
    fn min() -> i8 {
        i8::MIN
    }

    fn max() -> i8 {
        i8::MAX
    }

    fn from_f64(n: f64) -> i8 {
        n as i8
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl Sample for f32 {
    fn min() -> f32 {
         -1.0
    }

    fn max() -> f32 {
        1.0
    }

    fn from_f64(n: f64) -> f32 {
        n as f32
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}

*/
