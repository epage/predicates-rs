use std::ffi;
use std::fmt;
use std::str;

use Predicate;

/// Predicate adaper that trims the variable being tested.
///
/// This is created by `pred.trim()`.
#[derive(Clone, Debug)]
pub struct TrimPredicate<P>
where
    P: for<'a> Predicate<&'a str>,
{
    p: P,
}

impl<'a, P> Predicate<&'a str> for TrimPredicate<P>
where
    P: for<'b> Predicate<&'b str>,
{
    fn eval(&self, variable: &'a str) -> bool {
        self.p.eval(variable.trim())
    }
}

impl<P> fmt::Display for TrimPredicate<P>
where
    P: for<'a> Predicate<&'a str>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.p)
    }
}

/// Predicate adaper that converts a `str` predicate to byte predicate.
///
/// This is created by `pred.from_utf8()`.
#[derive(Clone, Debug)]
pub struct Utf8Predicate<P>
where
    P: for<'a> Predicate<&'a str>,
{
    p: P,
}

impl<'a, P> Predicate<&'a ffi::OsStr> for Utf8Predicate<P>
where
    P: for<'b> Predicate<&'b str>,
{
    fn eval(&self, variable: &'a ffi::OsStr) -> bool {
        variable.to_str().map(|s| self.p.eval(s)).unwrap_or(false)
    }
}

impl<'a, P> Predicate<&'a [u8]> for Utf8Predicate<P>
where
    P: for<'b> Predicate<&'b str>,
{
    fn eval(&self, variable: &'a [u8]) -> bool {
        str::from_utf8(variable)
            .map(|s| self.p.eval(s))
            .unwrap_or(false)
    }
}

impl<P> fmt::Display for Utf8Predicate<P>
where
    P: for<'a> Predicate<&'a str>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.p)
    }
}

/// `Predicate` extension adapting a `str` Predicate.
pub trait PredicateStrExt<'a>
where
    Self: Predicate<&'a str>,
    Self: Sized,
{
    /// Returns a `TrimPredicate` that ensures the data passed to `Self` is trimmed.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::str::is_empty().trim();
    /// assert_eq!(true, predicate_fn.eval("    "));
    /// assert_eq!(false, predicate_fn.eval("    Hello    "));
    /// ```
    fn trim(self) -> TrimPredicate<Self> {
        TrimPredicate { p: self }
    }

    /// Returns a `Utf8Predicate` that adapts `Self` to a `[u8]` `Predicate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    /// use std::ffi::OsStr;
    ///
    /// let predicate_fn = predicate::str::is_empty().not().from_utf8();
    /// assert_eq!(true, predicate_fn.eval(OsStr::new("Hello")));
    /// assert_eq!(false, predicate_fn.eval(OsStr::new("")));
    /// let variable: &[u8] = b"";
    /// assert_eq!(false, predicate_fn.eval(variable));
    /// ```
    fn from_utf8(self) -> Utf8Predicate<Self> {
        Utf8Predicate { p: self }
    }
}

impl<'a, P> PredicateStrExt<'a> for P
where
    P: Predicate<&'a str>,
{
}
