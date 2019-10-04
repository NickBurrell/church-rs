use memchr;
use nom::{Compare, CompareResult, FindSubstring, InputIter, InputLength, Offset, Slice};
use std::{
    iter::Enumerate,
    ops::{Range, RangeFrom, RangeFull, RangeTo},
    slice::Iter,
};
macro_rules! tokens {
    {
        ($name:ident: {
            value: $value:expr,
            desc: $description:expr,
        },*)
    } => {

        $(
            #[doc=$documentation]
            const $name: &'static[u8] = $value;
        )*
    };
    {
        ($name:ident: {
            value: $value:expr,
        },*)
    } => {
        $(
        const $name: &'static[u8] = $value;
        )*
    };
    {
        ($name:ident: {
            $value:expr,
            $description:expr,
        },*)
    } => {
        $(
            #[doc=$documentation]
            const $name: &'static[u8] = $value;
        )*
    };
    {
        ($name:ident: {
            $value:expr
        },*)
    } => {
        $(
            const $name: &'static[u8] = $value;
        )*
    };
}

#[derive(Debug, PartialEq)]
pub struct Token<'a, T> {
    pub value: T,
    pub span: Span<'a>,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span<'a> {
    pub offset: usize,
    pub line: u32,
    pub column: u32,
    slice: &'a [u8],
}

impl<'a> Span<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Span {
            offset: 0,
            line: 1,
            column: 1,
            slice: input,
        }
    }

    pub fn new_at(input: &'a [u8], offset: usize, line: u32, column: u32) -> Self {
        Span {
            offset: offset,
            line: line,
            column: column,
            slice: input,
        }
    }

    pub fn empty() -> Self {
        Self::new(b"")
    }

    pub fn as_slice(&self) -> &'a [u8] {
        self.slice
    }
}

impl<'a> InputLength for Span<'a> {
    fn input_len(&self) -> usize {
        self.slice.len()
    }
}

impl<'a> InputIter for Span<'a> {
    type Item = u8;
    type Iter = Enumerate<Iter<'a, u8>>;
    type IterElem = Iter<'a, u8>;

    fn iter_indices(&self) -> Self::Iter {
        self.slice.iter().enumerate()
    }
    fn iter_elements(&self) -> Self::IterElem {
        self.slice.iter()
    }
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.slice.iter().position(|x| predicate(*x))
    }
    fn slice_index(&self, count: usize) -> Option<usize> {
        if self.slice.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}

impl<'a, 'b> FindSubstring<&'b [u8]> for Span<'a> {
    fn find_substring(&self, substring: &'b [u8]) -> Option<usize> {
        let substring_length = substring.len();
        if substring_length == 0 {
            None
        } else if substring_length == 1 {
            memchr::memchr(substring[0], self.slice)
        } else {
            let max = self.slice.len() - substring_length;
            let mut offset = 0;
            let mut haystack = self.slice;

            while let Some(position) = memchr::memchr(substring[0], haystack) {
                offset += position;
                if offset > max {
                    return None;
                }
                if &haystack[position..position + substring_length] == substring {
                    return Some(offset);
                }
                haystack = &haystack[position + 1..];
                offset += 1;
            }
            None
        }
    }
}

impl<'a, 'b> Compare<&'b [u8]> for Span<'a> {
    fn compare(&self, element: &'b [u8]) -> CompareResult {
        self.slice.compare(element)
    }
    fn compare_no_case(&self, element: &'b [u8]) -> CompareResult {
        self.slice.compare_no_case(element)
    }
}
