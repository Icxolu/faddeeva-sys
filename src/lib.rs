#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use num_complex::Complex;
use std::marker::PhantomData;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl<T> From<Complex<T>> for std_complex<T> {
    fn from(z: Complex<T>) -> Self {
        let val_base = std__Complex_value {
            _Val: [z.re, z.im],
            _phantom_0: PhantomData,
        };
        let base = std__Complex_base {
            _base: val_base,
            _phantom_0: PhantomData,
        };
        Self {
            _base: base,
            _phantom_0: PhantomData,
        }
    }
}

impl<T> From<std_complex<T>> for Complex<T> {
    fn from(z: std_complex<T>) -> Self {
        let [re, im] = z._base._base._Val;
        Self { re, im }
    }
}
