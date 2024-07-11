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
            Some(_) => unreachable!("The above arms cover all possible cases"),
            None => {},
        }
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    /* In release mode, reaching this macro leads to UB  */
    #[cfg(debug_assertions)]
    fn will_panic() {
        unreachable!();
    }
}
