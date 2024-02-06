use std::error::Error;

pub type DefaultError = color_eyre::Report;

pub type DefaultRes<T, E = DefaultError> = color_eyre::Result<T, E>;

/// Analogous to regular `map` but works on any type.
pub trait MapType<M> {
    /// Converts one type into another.
    fn map_type<N>(self, f: impl FnOnce(M) -> N) -> N;
}

impl<M> MapType<M> for M {
    fn map_type<N>(self, f: impl FnOnce(Self) -> N) -> N {
        f(self)
    }
}

/// Wraps type in `Option` and returns `None` if condition is true.
pub trait NoneIf<T> {
    /// Returns `None` on `cond == true`.
    fn none_if(self, cond: impl Fn(&T) -> bool) -> Option<T>;
}

impl<T> NoneIf<T> for T {
    fn none_if(self, cond: impl Fn(&Self) -> bool) -> Option<Self> {
        match cond(&self) {
            true => None,
            _ => Some(self),
        }
    }
}

/// Wraps type in `Result` and returns `Err` if condition is true.
pub trait ErrIf<T> {
    /// Returns a given error on `cond == true`.
    fn err_if<E: Error>(self, cond: impl Fn(&Self) -> bool, err: E) -> Result<T, E>;
}

impl<T> ErrIf<T> for T {
    fn err_if<E: Error>(self, cond: impl Fn(&Self) -> bool, err: E) -> Result<Self, E> {
        match cond(&self) {
            true => Err(err),
            _ => Ok(self),
        }
    }
}

/// Maps any value to `()`.
pub trait ForgetValue {
    /// Forgets any value.
    fn forget_val(self);
}

impl<T> ForgetValue for T {
    fn forget_val(self) {}
}

/// Wraps type in `Result`.
/// It exists to complement convenience of mapping any value into `Option`.
pub trait WrapInRes<T> {
    /// Wraps type in `Result::Ok`.
    fn in_ok<E>(self) -> Result<T, E>;
    /// Wraps type in `Result::Err`.
    fn in_err<O>(self) -> Result<O, T>;
}

impl<T> WrapInRes<T> for T {
    fn in_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
    fn in_err<O>(self) -> Result<O, Self> {
        Err(self)
    }
}

/// Swaps `Ok` and `Err`.
/// May be useful in rare cases where the difference between `Result::Ok` and
/// `Result::Err` is shallow, or when you have things like `Option::Some(Error)`.
pub trait SwapRes<T, E> {
    /// Swaps `Ok` and `Err` variants.
    fn swap_res(self) -> Result<T, E>;
}

impl<T, E> SwapRes<E, T> for Result<T, E> {
    fn swap_res(self) -> Result<E, T> {
        match self {
            Ok(t) => Err(t),
            Err(e) => Ok(e),
        }
    }
}

/// Turns `Option<T>` into `Result`, with respect to whether `T` should be
/// an `Ok` or an `Err`.
pub trait AddToRes<T> {
    /// Turns `Option<T>` into `Result<O, T>`.
    fn with_ok<O>(self, ok: O) -> Result<O, T>;
    /// Turns `Option<T>` into `Result<T, E>`.
    fn with_err<E>(self, err: E) -> Result<T, E>;
}

impl<T> AddToRes<T> for Option<T> {
    fn with_ok<O>(self, ok: O) -> Result<O, T> {
        match self {
            None => Ok(ok),
            Some(e) => Err(e),
        }
    }
    fn with_err<E>(self, err: E) -> Result<T, E> {
        self.ok_or(err)
    }
}

/// Mutates value and returns it back.
pub trait Mutate<T> {
    /// Mutates value and returns it back.
    fn mutate<R>(self, f: impl FnOnce(&mut T) -> R) -> T;
}

impl<T> Mutate<T> for T {
    fn mutate<R>(self, f: impl FnOnce(&mut Self) -> R) -> Self {
        let mut val = self;
        f(&mut val);
        val
    }
}
