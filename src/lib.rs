//! This crate defines an [unreachable!] macro that is conditionally
//! compiled depending on the build profile.
//!
//! If the build is **debug** , it translates to [core::unreachable]
//!
//! If the build is **release**, it translates to [core::hint::unreachable_unchecked]
//!
//! There are cases where [unreachable_unchecked](core::hint::unreachable_unchecked)
//! is faster that [unreachable](core::unreachable).
//! This macro uses the unchecked version on release mode, but still checks on debug
//! mode, allowing you to catch cases in which the unreachable code is reached.
//!
//! # Example
//! ```
//! use dbg_unreachable::unreachable;
//!
//! let a = Some(12_i32);
//! match a {
//!     Some(n) if n >= 0 => {},
//!     Some(n) if n < 0 => {},
//!     Some(_) => unreachable!("The two arms above cover all possible cases"),
//!     None => {},
//! }
//! ```
//! The piece of code above translates to this two pieces.
//!
//! **Debug**
//! ```
//! use dbg_unreachable::unreachable;
//!
//! let a = Some(12_i32);
//! match a {
//!     Some(n) if n >= 0 => {},
//!     Some(n) if n < 0 => {},
//!     Some(_) => core::unreachable!("The two arms above cover all possible cases"),
//!     None => {},
//! }
//! ```
//!
//! **Release**
//! ```
//! use dbg_unreachable::unreachable;
//!
//! let a = Some(12_i32);
//! match a {
//!     Some(n) if n >= 0 => {},
//!     Some(n) if n < 0 => {},
//!     Some(_) => unsafe { core::hint::unreachable_unchecked() },
//!     None => {},
//! }
//! ```

#![no_std]

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! unreachable {
    ( $($e:expr)? ) => {
        core::unreachable!($($e)?)
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! unreachable {
    ( $($e:expr)? ) => {
        unsafe { core::hint::unreachable_unchecked() }
    };
}

#[cfg(test)]
mod test {
    use super::unreachable;

    #[test]
    fn test() {
        let a = Some(12_i32);
        match a {
            Some(n) if n >= 0 => {},
            Some(n) if n < 0 => {},
            Some(_) => unreachable!("The two arms above cover all possible cases"),
            None => {},
        }
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    /* In release mode, reaching this macro leads to UB */
    #[cfg(debug_assertions)]
    fn will_panic() {
        unreachable!();
    }
}
