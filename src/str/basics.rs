// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use Predicate;

/// Predicate that checks for empty strings.
///
/// This is created by `predicates::str::is_empty`.
#[derive(Copy, Clone, Debug)]
pub struct IsEmptyPredicate {}

impl<'a> Predicate<&'a str> for IsEmptyPredicate {
    fn eval(&self, variable: &'a str) -> bool {
        variable.is_empty()
    }
}

impl fmt::Display for IsEmptyPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.is_empty()")
    }
}

/// Creates a new `Predicate` that ensures a str is empty
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::is_empty();
/// assert_eq!(true, predicate_fn.eval(""));
/// assert_eq!(false, predicate_fn.eval("Food World"));
/// ```
pub fn is_empty() -> IsEmptyPredicate {
    IsEmptyPredicate {}
}

/// Predicate checks start of str
///
/// This is created by `predicates::str::starts_with`.
#[derive(Clone, Debug)]
pub struct StartsWithPredicate {
    pattern: String,
}

impl<'a> Predicate<&'a str> for StartsWithPredicate {
    fn eval(&self, variable: &'a str) -> bool {
        variable.starts_with(&self.pattern)
    }
}

impl fmt::Display for StartsWithPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.starts_with({:?})", self.pattern)
    }
}

/// Creates a new `Predicate` that ensures a str starts with `pattern`
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::starts_with("Hello");
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Goodbye World"));
/// ```
pub fn starts_with<P>(pattern: P) -> StartsWithPredicate
where
    P: Into<String>,
{
    StartsWithPredicate {
        pattern: pattern.into(),
    }
}

/// Predicate checks end of str
///
/// This is created by `predicates::str::ends_with`.
#[derive(Clone, Debug)]
pub struct EndsWithPredicate {
    pattern: String,
}

impl<'a> Predicate<&'a str> for EndsWithPredicate {
    fn eval(&self, variable: &str) -> bool {
        variable.ends_with(&self.pattern)
    }
}

impl fmt::Display for EndsWithPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.ends_with({:?})", self.pattern)
    }
}

/// Creates a new `Predicate` that ensures a str ends with `pattern`
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::ends_with("World");
/// assert_eq!(true, predicate_fn.eval("Hello World"));
/// assert_eq!(false, predicate_fn.eval("Hello Moon"));
/// ```
pub fn ends_with<P>(pattern: P) -> EndsWithPredicate
where
    P: Into<String>,
{
    EndsWithPredicate {
        pattern: pattern.into(),
    }
}

/// Predicate that checks for patterns.
///
/// This is created by `predicates::str:contains`.
#[derive(Clone, Debug)]
pub struct ContainsPredicate {
    pattern: String,
}

impl ContainsPredicate {
    /// Require a specific count of matches.
    ///
    /// # Examples
    ///
    /// ```
    /// use predicates::prelude::*;
    ///
    /// let predicate_fn = predicate::str::contains("Two").count(2);
    /// assert_eq!(true, predicate_fn.eval("One Two Three Two One"));
    /// assert_eq!(false, predicate_fn.eval("One Two Three"));
    /// ```
    pub fn count(self, count: usize) -> MatchesPredicate {
        MatchesPredicate {
            pattern: self.pattern,
            count,
        }
    }
}

impl<'a> Predicate<&'a str> for ContainsPredicate {
    fn eval(&self, variable: &'a str) -> bool {
        variable.contains(&self.pattern)
    }
}

impl fmt::Display for ContainsPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.contains({:?})", self.pattern)
    }
}

/// Predicate that checks for repeated patterns.
///
/// This is created by `predicates::str::contains(...).count`.
#[derive(Clone, Debug)]
pub struct MatchesPredicate {
    pattern: String,
    count: usize,
}

impl<'a> Predicate<&'a str> for MatchesPredicate {
    fn eval(&self, variable: &'a str) -> bool {
        variable.matches(&self.pattern).count() == self.count
    }
}

impl fmt::Display for MatchesPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.contains({:?}, {})", self.pattern, self.count)
    }
}

/// Creates a new `Predicate` that ensures a str contains `pattern`
///
/// # Examples
///
/// ```
/// use predicates::prelude::*;
///
/// let predicate_fn = predicate::str::contains("Two");
/// assert_eq!(true, predicate_fn.eval("One Two Three"));
/// assert_eq!(false, predicate_fn.eval("Four Five Six"));
/// ```
pub fn contains<P>(pattern: P) -> ContainsPredicate
where
    P: Into<String>,
{
    ContainsPredicate {
        pattern: pattern.into(),
    }
}
