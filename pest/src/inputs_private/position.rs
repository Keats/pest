// pest. The Elegant Parser
// Copyright (C) 2017  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use super::input::Input;
use super::span;

#[derive(Debug, Eq, Ord)]
pub struct Position<I: Input> {
    input: Rc<Arc<I>>,
    pos:   usize
}

#[inline]
pub fn new<I: Input>(input: Rc<Arc<I>>, pos: usize) -> Position<I> {
    Position {
        input: input,
        pos:   pos
    }
}

impl<I: Input> Position<I> {
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn at_start(&self) -> bool {
        self.pos == 0
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos == self.input.len()
    }

    #[inline]
    pub fn span(self, other: Position<I>) -> span::Span<I> {
        if &**self.input as *const I == &**other.input as *const I {
            span::new(self.input, self.pos, other.pos)
        } else {
            panic!("Span created from positions coming from different inputs")
        }
    }

    #[inline]
    pub fn line_col(&self) -> (usize, usize) {
        unsafe { self.input.line_col(self.pos) }
    }

    #[inline]
    pub fn line_of(&self) -> &str {
        unsafe { self.input.line_of(self.pos) }
    }

    #[inline]
    pub fn skip(self, n: usize) -> Result<Position<I>, Position<I>> {
        let skipped = unsafe { self.input.skip(n, self.pos) };

        match skipped {
            Some(len) => Ok(new(self.input, self.pos + len)),
            None      => Err(self)
        }
    }

    #[inline]
    pub fn match_string(self, string: &str) -> Result<Position<I>, Position<I>> {
        if unsafe { self.input.match_string(string, self.pos) } {
            Ok(new(self.input, self.pos + string.len()))
        } else {
            Err(self)
        }
    }

    #[inline]
    pub fn match_insensitive(self, string: &str) -> Result<Position<I>, Position<I>> {
        if unsafe { self.input.match_insensitive(string, self.pos) } {
            Ok(new(self.input, self.pos + string.len()))
        } else {
            Err(self)
        }
    }

    #[inline]
    pub fn match_range(self, range: Range<char>) -> Result<Position<I>, Position<I>> {
        let len = unsafe { self.input.match_range(range, self.pos) };

        match len {
            Some(len) => Ok(new(self.input, self.pos + len)),
            None      => Err(self)
        }
    }

    #[inline]
    pub fn repeat<F>(self, mut f: F) -> Result<Position<I>, Position<I>>
        where F: FnMut(Position<I>) -> Result<Position<I>, Position<I>> {

        let result = f(self);

        match result {
            Ok(pos)  => pos.repeat(f),
            Err(pos) => Ok(pos)
        }
    }
}

impl<I: Input> Clone for Position<I> {
    fn clone(&self) -> Position<I> {
        new(self.input.clone(), self.pos)
    }
}

impl<I: Input> PartialEq for Position<I> {
    fn eq(&self, other: &Position<I>) -> bool {
        &**self.input as *const I == &**other.input as *const I &&
        self.pos == other.pos
    }
}

impl<I: Input> PartialOrd for Position<I> {
    fn partial_cmp(&self, other: &Position<I>) -> Option<Ordering> {
        if &**self.input as *const I == &**other.input as *const I {
            self.pos.partial_cmp(&other.pos)
        } else {
            None
        }
    }
}

impl<I: Input> Hash for Position<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&**self.input as *const I).hash(state);
        self.pos.hash(state);
    }
}