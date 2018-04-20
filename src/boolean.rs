// Copyright (c) 2017, Nick Stevens <nick@bitcurry.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Definition of boolean logic combinators over `Predicate`s.

use std::marker::PhantomData;

use Predicate;

/// Predicate that combines two `Predicate`s, returning the AND of the results.
///
/// This is created by the `Predicate::and` function.
#[derive(Debug)]
pub struct AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: Copy,
{
    a: M1,
    b: M2,
    _phantom: PhantomData<Item>,
}

impl<M1, M2, Item> AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: Copy,
{
    /// Create a new `AndPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> AndPredicate<M1, M2, Item> {
        AndPredicate {
            a: a,
            b: b,
            _phantom: PhantomData,
        }
    }
}

impl<M1, M2, Item> Predicate<Item> for AndPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: Copy,
{
    fn eval(&self, item: Item) -> bool {
        self.a.eval(item) && self.b.eval(item)
    }
}

/// Predicate that combines two `Predicate`s, returning the OR of the results.
///
/// This is created by the `Predicate::or` function.
#[derive(Debug)]
pub struct OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: Copy,
{
    a: M1,
    b: M2,
    _phantom: PhantomData<Item>,
}

impl<M1, M2, Item> OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: Copy,
{
    /// Create a new `OrPredicate` over predicates `a` and `b`.
    pub fn new(a: M1, b: M2) -> OrPredicate<M1, M2, Item> {
        OrPredicate {
            a: a,
            b: b,
            _phantom: PhantomData,
        }
    }
}

impl<M1, M2, Item> Predicate<Item> for OrPredicate<M1, M2, Item>
where
    M1: Predicate<Item>,
    M2: Predicate<Item>,
    Item: Copy,
{
    fn eval(&self, item: Item) -> bool {
        self.a.eval(item) || self.b.eval(item)
    }
}

/// Predicate that returns a `Predicate` taking the logical NOT of the result.
///
/// This is created by the `Predicate::not` function.
#[derive(Debug)]
pub struct NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: Copy,
{
    inner: M,
    _phantom: PhantomData<Item>,
}

impl<M, Item> NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: Copy,
{
    /// Create a new `NotPredicate` over predicate `inner`.
    pub fn new(inner: M) -> NotPredicate<M, Item> {
        NotPredicate {
            inner: inner,
            _phantom: PhantomData,
        }
    }
}

impl<M, Item> Predicate<Item> for NotPredicate<M, Item>
where
    M: Predicate<Item>,
    Item: Copy,
{
    fn eval(&self, item: Item) -> bool {
        !self.inner.eval(item)
    }
}
