// Test
#![feature(prelude_import)]
#![no_std]
#![feature(const_fn)]

#![feature(plugin)]
#![plugin(phf_macros)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate phf;

#[allow(unused_imports)]
#[macro_use]
extern crate nom;

#[allow(unused_imports)]
#[macro_use]
extern crate quick_error;

mod church {

    pub mod parser {
        use nom::{digit, IResult, alphanumeric, is_space};
        use phf::{Map};
        use std::str::FromStr;
        use std::clone::Clone;
        use super::error::*;
        use super::utils::*;
        mod primatives {
            use super::*;
            use super::super::error::*;
            fn add(v1: ChurchValue, v2: ChurchValue)
             -> Result<ChurchValue, ChurchEvalError> {
                match v1 {
                    ChurchValue::Number(x) => {
                        match v2 {
                            ChurchValue::Number(y) => {
                                Ok(ChurchValue::Number(x + y))
                            }
                            _ =>
                            Err(ChurchEvalError::TypeError(String::from("+"),
                                                           String::from(""),
                                                           String::from(""))),
                        }
                    }
                    _ =>
                    Err(ChurchEvalError::TypeError(String::from("+"),
                                                   String::from(""),
                                                   String::from(""))),
                }
            }
            fn sub(v1: ChurchValue, v2: ChurchValue)
             -> Result<ChurchValue, ChurchEvalError> {
                match v1 {
                    ChurchValue::Number(x) => {
                        match v2 {
                            ChurchValue::Number(y) => {
                                Ok(ChurchValue::Number(x - y))
                            }
                            _ =>
                            Err(ChurchEvalError::TypeError(String::from("-"),
                                                           String::from(""),
                                                           String::from(""))),
                        }
                    }
                    _ =>
                    Err(ChurchEvalError::TypeError(String::from("-"),
                                                   String::from(""),
                                                   String::from(""))),
                }
            }
            fn mul(v1: ChurchValue, v2: ChurchValue)
             -> Result<ChurchValue, ChurchEvalError> {
                match v1 {
                    ChurchValue::Number(x) => {
                        match v2 {
                            ChurchValue::Number(y) => {
                                Ok(ChurchValue::Number(x * y))
                            }
                            _ =>
                            Err(ChurchEvalError::TypeError(String::from("*"),
                                                           String::from(""),
                                                           String::from(""))),
                        }
                    }
                    _ =>
                    Err(ChurchEvalError::TypeError(String::from("*"),
                                                   String::from(""),
                                                   String::from(""))),
                }
            }
            fn div(v1: ChurchValue, v2: ChurchValue)
             -> Result<ChurchValue, ChurchEvalError> {
                match v1 {
                    ChurchValue::Number(x) => {
                        match v2 {
                            ChurchValue::Number(y) => {
                                Ok(ChurchValue::Number(x / y))
                            }
                            _ =>
                            Err(ChurchEvalError::TypeError(String::from("/"),
                                                           String::from(""),
                                                           String::from(""))),
                        }
                    }
                    _ =>
                    Err(ChurchEvalError::TypeError(String::from("/"),
                                                   String::from(""),
                                                   String::from(""))),
                }
            }
            fn exp<'a>(v1: ChurchValue, v2: ChurchValue)
             -> Result<ChurchValue, ChurchEvalError> {
                match v1 {
                    ChurchValue::Number(x) => {
                        match v2 {
                            ChurchValue::Number(y) => {
                                Ok(ChurchValue::Number(x.pow(y as u32)))
                            }
                            _ =>
                            Err(ChurchEvalError::TypeError(String::from("+"),
                                                           String::from(""),
                                                           String::from(""))),
                        }
                    }
                    _ =>
                    Err(ChurchEvalError::TypeError(String::from("+"),
                                                   String::from(""),
                                                   String::from(""))),
                }
            }
            fn modu<'a>(v1: ChurchValue, v2: ChurchValue)
             -> Result<ChurchValue, ChurchEvalError> {
                match v1 {
                    ChurchValue::Number(x) => {
                        match v2 {
                            ChurchValue::Number(y) => {
                                Ok(ChurchValue::Number(x % y))
                            }
                            _ =>
                            Err(ChurchEvalError::TypeError(String::from("+"),
                                                           String::from(""),
                                                           String::from(""))),
                        }
                    }
                    _ =>
                    Err(ChurchEvalError::TypeError(String::from("+"),
                                                   String::from(""),
                                                   String::from(""))),
                }
            }
            pub static PRIMATIVES:
                       Map<&'static str,
                           fn(ChurchValue, ChurchValue)
                               -> Result<ChurchValue, ChurchEvalError>> =
                ::phf::Map{key: 1897749892740154578u64,
                           disps:
                               ::phf::Slice::Static(&[(1u32, 0u32),
                                                      (1u32, 0u32)]),
                           entries:
                               ::phf::Slice::Static(&[("+", add), ("/", div),
                                                      ("^", exp), ("*", mul),
                                                      ("-", sub),
                                                      ("%", modu)]),};
        }
        #[structural_match]
        pub enum ChurchValue {
            Number(i16),
            Bool(bool),
            List(Box<Vec<ChurchValue>>),
            Func(String, Box<Vec<ChurchValue>>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ChurchValue {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&ChurchValue::Number(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Number");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&ChurchValue::Bool(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Bool");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&ChurchValue::List(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("List");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&ChurchValue::Func(ref __self_0, ref __self_1),) => {
                        let mut builder = __arg_0.debug_tuple("Func");
                        let _ = builder.field(&&(*__self_0));
                        let _ = builder.field(&&(*__self_1));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::Eq for ChurchValue {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<i16>;
                    let _: ::std::cmp::AssertParamIsEq<bool>;
                    let _: ::std::cmp::AssertParamIsEq<Box<Vec<ChurchValue>>>;
                    let _: ::std::cmp::AssertParamIsEq<String>;
                    let _: ::std::cmp::AssertParamIsEq<Box<Vec<ChurchValue>>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ChurchValue {
            #[inline]
            fn eq(&self, __arg_0: &ChurchValue) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) {
                            (&ChurchValue::Number(ref __self_0),
                             &ChurchValue::Number(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            (&ChurchValue::Bool(ref __self_0),
                             &ChurchValue::Bool(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            (&ChurchValue::List(ref __self_0),
                             &ChurchValue::List(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            (&ChurchValue::Func(ref __self_0, ref __self_1),
                             &ChurchValue::Func(ref __arg_1_0, ref __arg_1_1))
                            =>
                            true && (*__self_0) == (*__arg_1_0) &&
                                (*__self_1) == (*__arg_1_1),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { false }
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &ChurchValue) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) {
                            (&ChurchValue::Number(ref __self_0),
                             &ChurchValue::Number(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            (&ChurchValue::Bool(ref __self_0),
                             &ChurchValue::Bool(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            (&ChurchValue::List(ref __self_0),
                             &ChurchValue::List(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            (&ChurchValue::Func(ref __self_0, ref __self_1),
                             &ChurchValue::Func(ref __arg_1_0, ref __arg_1_1))
                            =>
                            false || (*__self_0) != (*__arg_1_0) ||
                                (*__self_1) != (*__arg_1_1),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { true }
                }
            }
        }
        impl ChurchValue {
            fn parse_string_to_bool(input: &str)
             -> Result<Self, ChurchParseError> {
                match input {
                    "#t" => Ok(ChurchValue::Bool(true)),
                    "#f" => Ok(ChurchValue::Bool(false)),
                    _ => Err(ChurchParseError::BoolParseError),
                }
            }
            fn parse_string_to_i16(input: &str)
             -> Result<Self, ChurchParseError> {
                let parsed_int = i16::from_str(input);
                match parsed_int {
                    Ok(value) => Ok(ChurchValue::Number(value)),
                    Err(_) => Err(ChurchParseError::IntParseError),
                }
            }
            fn parse_vec_to_church_value<T: ToString>(v: Vec<T>)
             -> Result<ChurchValue, ChurchParseError> {
                let mut out_vec: Vec<ChurchValue> = Vec::new();
                let mut output_value: Result<ChurchValue, ChurchParseError> =
                    Ok(ChurchValue::Number(0));
                for i in v.into_iter() {
                    let str_ref = i.to_string();
                    let elem = church_parse(&str_ref);
                    let index = out_vec.len();
                    match elem {
                        IResult::Done(_, val) =>
                        out_vec.insert(index, val.unwrap()),
                        _ =>
                        output_value = Err(ChurchParseError::ListParseError),
                    };
                };
                match output_value {
                    Err(_) => { }
                    _ =>
                    output_value = Ok(ChurchValue::List(Box::new(out_vec))),
                }
                output_value
            }
            fn parse_function_application_to_church_value(fn_name: &str,
                                                          args: Vec<&str>)
             -> Result<ChurchValue, ChurchParseError> {
                let mut arg_list: Vec<ChurchValue> = Vec::new();
                let mut break_for_inner_error: bool = false;
                for arg in args {
                    let temp_val = church_parse(arg);
                    match temp_val {
                        IResult::Done(_, _val) => {
                            match _val {
                                Ok(val) => { arg_list.push(val); }
                                Err(err) => { return Err(err); }
                            }
                        }
                        _ => { }
                    }
                }
                Ok(ChurchValue::Func(fn_name.to_string(), Box::new(arg_list)))
            }
        }
        unsafe impl ::std::marker::Sync for ChurchValue { }
        impl ToString for ChurchValue {
            fn to_string(&self) -> String {
                match self {
                    &ChurchValue::Number(out) => out.to_string(),
                    &ChurchValue::Bool(out) => out.to_string(),
                    &ChurchValue::List(ref out) =>
                    vec_ref_to_string(&*out, ", "),
                    &ChurchValue::Func(ref fn_name, ref args) => {
                        let mut out_str = fn_name.clone();
                        out_str.push_str(vec_ref_to_string(args,
                                                           " ").as_str());
                        out_str
                    }
                }
            }
        }
        impl Clone for ChurchValue {
            fn clone(&self) -> ChurchValue {
                match self {
                    &ChurchValue::Number(val) => ChurchValue::Number(val),
                    &ChurchValue::Bool(val) => ChurchValue::Bool(val),
                    &ChurchValue::List(ref val) =>
                    ChurchValue::List(val.clone()),
                    &ChurchValue::Func(ref fn_name, ref args) =>
                    ChurchValue::Func(fn_name.to_owned(), args.clone()),
                }
            }
        }
        #[allow(unused_variables)]
        fn parse_number(i: &str)
         -> ::IResult<&str, Result<ChurchValue, ChurchParseError>, u32> {
            {
                pub fn _unify<T, R, F: FnOnce(T) -> R>(f: F, t: T) -> R {
                    f(t)
                }
                match digit(i) {
                    ::IResult::Error(e) => ::IResult::Error(e),
                    ::IResult::Incomplete(::Needed::Unknown) =>
                    ::IResult::Incomplete(::Needed::Unknown),
                    ::IResult::Incomplete(::Needed::Size(i)) =>
                    ::IResult::Incomplete(::Needed::Size(i)),
                    ::IResult::Done(i, o) =>
                    ::IResult::Done(i,
                                    _unify(ChurchValue::parse_string_to_i16,
                                           o)),
                }
            }
        }
        #[allow(unused_variables)]
        fn parse_bool(i: &str)
         -> ::IResult<&str, Result<ChurchValue, ChurchParseError>, u32> {
            {
                pub fn _unify<T, R, F: FnOnce(T) -> R>(f: F, t: T) -> R {
                    f(t)
                }
                match {
                          {
                              let i_ = i.clone();
                              let res =
                                  {
                                      use ::{Compare, CompareResult,
                                             InputLength, Slice};
                                      let res: ::IResult<_, _> =
                                          match (i_).compare("#t") {
                                              CompareResult::Ok => {
                                                  let blen = "#t".input_len();
                                                  ::IResult::Done(i_.slice(blen..),
                                                                  i_.slice(..blen))
                                              }
                                              CompareResult::Incomplete => {
                                                  ::IResult::Incomplete(::Needed::Size("#t".input_len()))
                                              }
                                              CompareResult::Error => {
                                                  ::IResult::Error(::ErrorKind::Tag)
                                              }
                                          };
                                      res
                                  };
                              match res {
                                  ::IResult::Done(_, _) => res,
                                  ::IResult::Incomplete(_) => res,
                                  ::IResult::Error(e) => {
                                      let out =
                                          {
                                              let i_ = i.clone();
                                              let res =
                                                  {
                                                      use ::{Compare,
                                                             CompareResult,
                                                             InputLength,
                                                             Slice};
                                                      let res:
                                                              ::IResult<_,
                                                                        _> =
                                                          match (i_).compare("#f")
                                                              {
                                                              CompareResult::Ok
                                                              => {
                                                                  let blen =
                                                                      "#f".input_len();
                                                                  ::IResult::Done(i_.slice(blen..),
                                                                                  i_.slice(..blen))
                                                              }
                                                              CompareResult::Incomplete
                                                              => {
                                                                  ::IResult::Incomplete(::Needed::Size("#f".input_len()))
                                                              }
                                                              CompareResult::Error
                                                              => {
                                                                  ::IResult::Error(::ErrorKind::Tag)
                                                              }
                                                          };
                                                      res
                                                  };
                                              match res {
                                                  ::IResult::Done(_, _) =>
                                                  res,
                                                  ::IResult::Incomplete(_) =>
                                                  res,
                                                  ::IResult::Error(e) => {
                                                      let out =
                                                          ::IResult::Error(::ErrorKind::Alt);
                                                      fn unify_types<T>(_: &T,
                                                                        _:
                                                                            &T) {
                                                      }
                                                      if let ::IResult::Error(ref e2)
                                                             = out {
                                                          unify_types(&e, e2);
                                                      }
                                                      out
                                                  }
                                              }
                                          };
                                      fn unify_types<T>(_: &T, _: &T) { }
                                      if let ::IResult::Error(ref e2) = out {
                                          unify_types(&e, e2);
                                      }
                                      out
                                  }
                              }
                          }
                      } {
                    ::IResult::Error(e) => ::IResult::Error(e),
                    ::IResult::Incomplete(::Needed::Unknown) =>
                    ::IResult::Incomplete(::Needed::Unknown),
                    ::IResult::Incomplete(::Needed::Size(i)) =>
                    ::IResult::Incomplete(::Needed::Size(i)),
                    ::IResult::Done(i, o) =>
                    ::IResult::Done(i,
                                    _unify(ChurchValue::parse_string_to_bool,
                                           o)),
                }
            }
        }
        #[allow(unused_variables)]
        fn alphanumeric_or_bool(i: &str) -> ::IResult<&str, &str, u32> {
            {
                {
                    let i_ = i.clone();
                    let res = alphanumeric(i_);
                    match res {
                        ::IResult::Done(_, _) => res,
                        ::IResult::Incomplete(_) => res,
                        ::IResult::Error(e) => {
                            let out =
                                {
                                    let i_ = i.clone();
                                    let res =
                                        {
                                            {
                                                let i_ = i_.clone();
                                                let res =
                                                    {
                                                        use ::{Compare,
                                                               CompareResult,
                                                               InputLength,
                                                               Slice};
                                                        let res:
                                                                ::IResult<_,
                                                                          _> =
                                                            match (i_).compare("#t")
                                                                {
                                                                CompareResult::Ok
                                                                => {
                                                                    let blen =
                                                                        "#t".input_len();
                                                                    ::IResult::Done(i_.slice(blen..),
                                                                                    i_.slice(..blen))
                                                                }
                                                                CompareResult::Incomplete
                                                                => {
                                                                    ::IResult::Incomplete(::Needed::Size("#t".input_len()))
                                                                }
                                                                CompareResult::Error
                                                                => {
                                                                    ::IResult::Error(::ErrorKind::Tag)
                                                                }
                                                            };
                                                        res
                                                    };
                                                match res {
                                                    ::IResult::Done(_, _) =>
                                                    res,
                                                    ::IResult::Incomplete(_)
                                                    => res,
                                                    ::IResult::Error(e) => {
                                                        let out =
                                                            {
                                                                let i_ =
                                                                    i_.clone();
                                                                let res =
                                                                    {
                                                                        use ::{Compare,
                                                                               CompareResult,
                                                                               InputLength,
                                                                               Slice};
                                                                        let res:
                                                                                ::IResult<_,
                                                                                          _> =
                                                                            match (i_).compare("#f")
                                                                                {
                                                                                CompareResult::Ok
                                                                                =>
                                                                                {
                                                                                    let blen =
                                                                                        "#f".input_len();
                                                                                    ::IResult::Done(i_.slice(blen..),
                                                                                                    i_.slice(..blen))
                                                                                }
                                                                                CompareResult::Incomplete
                                                                                =>
                                                                                {
                                                                                    ::IResult::Incomplete(::Needed::Size("#f".input_len()))
                                                                                }
                                                                                CompareResult::Error
                                                                                =>
                                                                                {
                                                                                    ::IResult::Error(::ErrorKind::Tag)
                                                                                }
                                                                            };
                                                                        res
                                                                    };
                                                                match res {
                                                                    ::IResult::Done(_,
                                                                                    _)
                                                                    => res,
                                                                    ::IResult::Incomplete(_)
                                                                    => res,
                                                                    ::IResult::Error(e)
                                                                    => {
                                                                        let out =
                                                                            ::IResult::Error(::ErrorKind::Alt);
                                                                        fn unify_types<T>(_:
                                                                                              &T,
                                                                                          _:
                                                                                              &T) {
                                                                        }
                                                                        if let ::IResult::Error(ref e2)
                                                                               =
                                                                               out
                                                                               {
                                                                            unify_types(&e,
                                                                                        e2);
                                                                        }
                                                                        out
                                                                    }
                                                                }
                                                            };
                                                        fn unify_types<T>(_:
                                                                              &T,
                                                                          _:
                                                                              &T) {
                                                        }
                                                        if let ::IResult::Error(ref e2)
                                                               = out {
                                                            unify_types(&e,
                                                                        e2);
                                                        }
                                                        out
                                                    }
                                                }
                                            }
                                        };
                                    match res {
                                        ::IResult::Done(_, _) => res,
                                        ::IResult::Incomplete(_) => res,
                                        ::IResult::Error(e) => {
                                            let out =
                                                ::IResult::Error(::ErrorKind::Alt);
                                            fn unify_types<T>(_: &T, _: &T) {
                                            }
                                            if let ::IResult::Error(ref e2) =
                                                   out {
                                                unify_types(&e, e2);
                                            }
                                            out
                                        }
                                    }
                                };
                            fn unify_types<T>(_: &T, _: &T) { }
                            if let ::IResult::Error(ref e2) = out {
                                unify_types(&e, e2);
                            }
                            out
                        }
                    }
                }
            }
        }
        #[allow(unused_variables)]
        fn parse_list(i: &str)
         -> ::IResult<&str, Result<ChurchValue, ChurchParseError>, u32> {
            {
                {
                    let i_ = i.clone();
                    match {
                              use ::{Compare, CompareResult, InputLength,
                                     Slice};
                              let res: ::IResult<_, _> =
                                  match (i_).compare("(") {
                                      CompareResult::Ok => {
                                          let blen = "(".input_len();
                                          ::IResult::Done(i_.slice(blen..),
                                                          i_.slice(..blen))
                                      }
                                      CompareResult::Incomplete => {
                                          ::IResult::Incomplete(::Needed::Size("(".input_len()))
                                      }
                                      CompareResult::Error => {
                                          ::IResult::Error(::ErrorKind::Tag)
                                      }
                                  };
                              res
                          } {
                        ::IResult::Error(e) => ::IResult::Error(e),
                        ::IResult::Incomplete(::Needed::Unknown) =>
                        ::IResult::Incomplete(::Needed::Unknown),
                        ::IResult::Incomplete(::Needed::Size(i)) => {
                            let (needed, overflowed) =
                                0usize.overflowing_add(i);
                            match overflowed {
                                true =>
                                ::IResult::Incomplete(::Needed::Unknown),
                                false =>
                                ::IResult::Incomplete(::Needed::Size(needed)),
                            }
                        }
                        ::IResult::Done(i, o) => {
                            let begin = o;
                            let i_ = i.clone();
                            {
                                let i_ = i_.clone();
                                match {
                                          use ::InputLength;
                                          let mut res =
                                              ::std::vec::Vec::new();
                                          let mut input = i_.clone();
                                          let input_ = input.clone();
                                          match alphanumeric_or_bool(input_) {
                                              ::IResult::Error(_) =>
                                              ::IResult::Done(input,
                                                              ::std::vec::Vec::new()),
                                              ::IResult::Incomplete(i) =>
                                              ::IResult::Incomplete(i),
                                              ::IResult::Done(i, o) => {
                                                  if i.input_len() ==
                                                         input.input_len() {
                                                      ::IResult::Error(::ErrorKind::SeparatedList)
                                                  } else {
                                                      res.push(o);
                                                      input = i;
                                                      let ret;
                                                      loop  {
                                                          let input_ =
                                                              input.clone();
                                                          match {
                                                                    use ::{Compare,
                                                                           CompareResult,
                                                                           InputLength,
                                                                           Slice};
                                                                    let res:
                                                                            ::IResult<_,
                                                                                      _> =
                                                                        match (input_).compare(",")
                                                                            {
                                                                            CompareResult::Ok
                                                                            =>
                                                                            {
                                                                                let blen =
                                                                                    ",".input_len();
                                                                                ::IResult::Done(input_.slice(blen..),
                                                                                                input_.slice(..blen))
                                                                            }
                                                                            CompareResult::Incomplete
                                                                            =>
                                                                            {
                                                                                ::IResult::Incomplete(::Needed::Size(",".input_len()))
                                                                            }
                                                                            CompareResult::Error
                                                                            =>
                                                                            {
                                                                                ::IResult::Error(::ErrorKind::Tag)
                                                                            }
                                                                        };
                                                                    res
                                                                } {
                                                              ::IResult::Error(_)
                                                              => {
                                                                  ret =
                                                                      ::IResult::Done(input,
                                                                                      res);
                                                                  break ;
                                                              }
                                                              ::IResult::Incomplete(::Needed::Unknown)
                                                              => {
                                                                  ret =
                                                                      ::IResult::Incomplete(::Needed::Unknown);
                                                                  break ;
                                                              }
                                                              ::IResult::Incomplete(::Needed::Size(needed))
                                                              => {
                                                                  let (size,
                                                                       overflowed) =
                                                                      needed.overflowing_add((i_).input_len()
                                                                                                 -
                                                                                                 input.input_len());
                                                                  ret =
                                                                      match overflowed
                                                                          {
                                                                          true
                                                                          =>
                                                                          ::IResult::Incomplete(::Needed::Unknown),
                                                                          false
                                                                          =>
                                                                          ::IResult::Incomplete(::Needed::Size(size)),
                                                                      };
                                                                  break ;
                                                              }
                                                              ::IResult::Done(i2,
                                                                              _)
                                                              => {
                                                                  let i2_len =
                                                                      i2.input_len();
                                                                  if i2_len ==
                                                                         input.input_len()
                                                                     {
                                                                      ret =
                                                                          ::IResult::Done(input,
                                                                                          res);
                                                                      break ;
                                                                  }
                                                                  match alphanumeric_or_bool(i2)
                                                                      {
                                                                      ::IResult::Error(_)
                                                                      => {
                                                                          ret
                                                                              =
                                                                              ::IResult::Done(input,
                                                                                              res);
                                                                          break
                                                                              ;
                                                                      }
                                                                      ::IResult::Incomplete(::Needed::Unknown)
                                                                      => {
                                                                          ret
                                                                              =
                                                                              ::IResult::Incomplete(::Needed::Unknown);
                                                                          break
                                                                              ;
                                                                      }
                                                                      ::IResult::Incomplete(::Needed::Size(needed))
                                                                      => {
                                                                          let (size,
                                                                               overflowed) =
                                                                              needed.overflowing_add((i_).input_len()
                                                                                                         -
                                                                                                         i2_len);
                                                                          ret
                                                                              =
                                                                              match overflowed
                                                                                  {
                                                                                  true
                                                                                  =>
                                                                                  ::IResult::Incomplete(::Needed::Unknown),
                                                                                  false
                                                                                  =>
                                                                                  ::IResult::Incomplete(::Needed::Size(size)),
                                                                              };
                                                                          break
                                                                              ;
                                                                      }
                                                                      ::IResult::Done(i3,
                                                                                      o3)
                                                                      => {
                                                                          if i3.input_len()
                                                                                 ==
                                                                                 i2_len
                                                                             {
                                                                              ret
                                                                                  =
                                                                                  ::IResult::Done(input,
                                                                                                  res);
                                                                              break
                                                                                  ;
                                                                          }
                                                                          res.push(o3);
                                                                          input
                                                                              =
                                                                              i3;
                                                                      }
                                                                  }
                                                              }
                                                          }
                                                      }
                                                      ret
                                                  }
                                              }
                                          }
                                      } {
                                    ::IResult::Error(e) =>
                                    ::IResult::Error(e),
                                    ::IResult::Incomplete(::Needed::Unknown)
                                    =>
                                    ::IResult::Incomplete(::Needed::Unknown),
                                    ::IResult::Incomplete(::Needed::Size(i))
                                    => {
                                        let (needed, overflowed) =
                                            (0usize +
                                                 (::InputLength::input_len(&(i))
                                                      -
                                                      ::InputLength::input_len(&i))).overflowing_add(i);
                                        match overflowed {
                                            true =>
                                            ::IResult::Incomplete(::Needed::Unknown),
                                            false =>
                                            ::IResult::Incomplete(::Needed::Size(needed)),
                                        }
                                    }
                                    ::IResult::Done(i, o) => {
                                        let output = o;
                                        let i_ = i.clone();
                                        {
                                            let i_ = i_.clone();
                                            match {
                                                      use ::{Compare,
                                                             CompareResult,
                                                             InputLength,
                                                             Slice};
                                                      let res:
                                                              ::IResult<_,
                                                                        _> =
                                                          match (i_).compare(")")
                                                              {
                                                              CompareResult::Ok
                                                              => {
                                                                  let blen =
                                                                      ")".input_len();
                                                                  ::IResult::Done(i_.slice(blen..),
                                                                                  i_.slice(..blen))
                                                              }
                                                              CompareResult::Incomplete
                                                              => {
                                                                  ::IResult::Incomplete(::Needed::Size(")".input_len()))
                                                              }
                                                              CompareResult::Error
                                                              => {
                                                                  ::IResult::Error(::ErrorKind::Tag)
                                                              }
                                                          };
                                                      res
                                                  } {
                                                ::IResult::Error(e) =>
                                                ::IResult::Error(e),
                                                ::IResult::Incomplete(::Needed::Unknown)
                                                =>
                                                ::IResult::Incomplete(::Needed::Unknown),
                                                ::IResult::Incomplete(::Needed::Size(i))
                                                => {
                                                    let (needed, overflowed) =
                                                        (0usize +
                                                             (::InputLength::input_len(&(i))
                                                                  -
                                                                  ::InputLength::input_len(&i))
                                                             +
                                                             (::InputLength::input_len(&(i_))
                                                                  -
                                                                  ::InputLength::input_len(&i))).overflowing_add(i);
                                                    match overflowed {
                                                        true =>
                                                        ::IResult::Incomplete(::Needed::Unknown),
                                                        false =>
                                                        ::IResult::Incomplete(::Needed::Size(needed)),
                                                    }
                                                }
                                                ::IResult::Done(i, o) => {
                                                    let end = o;
                                                    let i_ = i.clone();
                                                    ::IResult::Done(i_,
                                                                    (ChurchValue::parse_vec_to_church_value(output)))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[allow(unused_variables)]
        fn church_symbol(i: &str) -> ::IResult<&str, char, u32> {
            {
                use ::Slice;
                use ::AsChar;
                use ::FindToken;
                use ::InputIter;
                match (i).iter_elements().next().map(|c|
                                                         {
                                                             (c,
                                                              c.find_token("!#$%&|*=-/:<=>?@^_~"))
                                                         }) {
                    None => ::IResult::Incomplete::<_, _>(::Needed::Size(1)),
                    Some((_, false)) => ::IResult::Error(::ErrorKind::OneOf),
                    Some((c, true)) =>
                    ::IResult::Done(i.slice(c.len()..),
                                    i.iter_elements().next().unwrap().as_char()),
                }
            }
        }
        #[allow(unused_variables)]
        fn church_primatives(i: &str) -> ::IResult<&str, char, u32> {
            {
                use ::Slice;
                use ::AsChar;
                use ::FindToken;
                use ::InputIter;
                match (i).iter_elements().next().map(|c|
                                                         {
                                                             (c,
                                                              c.find_token("+-*/^%"))
                                                         }) {
                    None => ::IResult::Incomplete::<_, _>(::Needed::Size(1)),
                    Some((_, false)) => ::IResult::Error(::ErrorKind::OneOf),
                    Some((c, true)) =>
                    ::IResult::Done(i.slice(c.len()..),
                                    i.iter_elements().next().unwrap().as_char()),
                }
            }
        }
        #[allow(unused_variables)]
        fn church_parse(i: &str)
         -> ::IResult<&str, Result<ChurchValue, ChurchParseError>, u32> {
            {
                {
                    let i_ = i.clone();
                    let res =
                        {
                            {
                                let i_ = i_.clone();
                                let res =
                                    {
                                        let i_ = i_.clone();
                                        match parse_bool(i_) {
                                            ::IResult::Done(i, o) =>
                                            ::IResult::Done(i, o),
                                            ::IResult::Error(e) =>
                                            ::IResult::Error(e),
                                            ::IResult::Incomplete(_) => {
                                                ::IResult::Error(::ErrorKind::Complete)
                                            }
                                        }
                                    };
                                match res {
                                    ::IResult::Done(_, _) => res,
                                    ::IResult::Incomplete(_) => res,
                                    ::IResult::Error(e) => {
                                        let out =
                                            {
                                                let i_ = i_.clone();
                                                let res =
                                                    {
                                                        let i_ = i_.clone();
                                                        match parse_number(i_)
                                                            {
                                                            ::IResult::Done(i,
                                                                            o)
                                                            =>
                                                            ::IResult::Done(i,
                                                                            o),
                                                            ::IResult::Error(e)
                                                            =>
                                                            ::IResult::Error(e),
                                                            ::IResult::Incomplete(_)
                                                            => {
                                                                ::IResult::Error(::ErrorKind::Complete)
                                                            }
                                                        }
                                                    };
                                                match res {
                                                    ::IResult::Done(_, _) =>
                                                    res,
                                                    ::IResult::Incomplete(_)
                                                    => res,
                                                    ::IResult::Error(e) => {
                                                        let out =
                                                            ::IResult::Error(::ErrorKind::Alt);
                                                        fn unify_types<T>(_:
                                                                              &T,
                                                                          _:
                                                                              &T) {
                                                        }
                                                        if let ::IResult::Error(ref e2)
                                                               = out {
                                                            unify_types(&e,
                                                                        e2);
                                                        }
                                                        out
                                                    }
                                                }
                                            };
                                        fn unify_types<T>(_: &T, _: &T) { }
                                        if let ::IResult::Error(ref e2) = out
                                               {
                                            unify_types(&e, e2);
                                        }
                                        out
                                    }
                                }
                            }
                        };
                    match res {
                        ::IResult::Done(_, _) => res,
                        ::IResult::Incomplete(_) => res,
                        ::IResult::Error(e) => {
                            let out =
                                {
                                    let i_ = i.clone();
                                    let res =
                                        {
                                            let i_ = i_.clone();
                                            match parse_list(i_) {
                                                ::IResult::Done(i, o) =>
                                                ::IResult::Done(i, o),
                                                ::IResult::Error(e) =>
                                                ::IResult::Error(e),
                                                ::IResult::Incomplete(_) => {
                                                    ::IResult::Error(::ErrorKind::Complete)
                                                }
                                            }
                                        };
                                    match res {
                                        ::IResult::Done(_, _) => res,
                                        ::IResult::Incomplete(_) => res,
                                        ::IResult::Error(e) => {
                                            let out =
                                                ::IResult::Error(::ErrorKind::Alt);
                                            fn unify_types<T>(_: &T, _: &T) {
                                            }
                                            if let ::IResult::Error(ref e2) =
                                                   out {
                                                unify_types(&e, e2);
                                            }
                                            out
                                        }
                                    }
                                };
                            fn unify_types<T>(_: &T, _: &T) { }
                            if let ::IResult::Error(ref e2) = out {
                                unify_types(&e, e2);
                            }
                            out
                        }
                    }
                }
            }
        }
        #[allow(unused_variables)]
        fn church_parse_fn(i: &str)
         -> ::IResult<&str, Result<ChurchValue, ChurchParseError>, u32> {
            {
                {
                    let i_ = i.clone();
                    match {
                              let input = i_;
                              use ::InputLength;
                              use ::InputIter;
                              use ::Slice;
                              match input.position(|c| is_space(c)) {
                                  Some(n) =>
                                  ::IResult::Done(input.slice(n..),
                                                  input.slice(..n)),
                                  None =>
                                  ::IResult::Done(input.slice(input.input_len()..),
                                                  input),
                              }
                          } {
                        ::IResult::Error(e) => ::IResult::Error(e),
                        ::IResult::Incomplete(::Needed::Unknown) =>
                        ::IResult::Incomplete(::Needed::Unknown),
                        ::IResult::Incomplete(::Needed::Size(i)) => {
                            let (needed, overflowed) =
                                0usize.overflowing_add(i);
                            match overflowed {
                                true =>
                                ::IResult::Incomplete(::Needed::Unknown),
                                false =>
                                ::IResult::Incomplete(::Needed::Size(needed)),
                            }
                        }
                        ::IResult::Done(i, o) => {
                            let function_name = o;
                            let i_ = i.clone();
                            {
                                let i_ = i_.clone();
                                match {
                                          use ::InputLength;
                                          let mut res =
                                              ::std::vec::Vec::new();
                                          let mut input = i_.clone();
                                          let input_ = input.clone();
                                          match alphanumeric_or_bool(input_) {
                                              ::IResult::Error(_) =>
                                              ::IResult::Done(input,
                                                              ::std::vec::Vec::new()),
                                              ::IResult::Incomplete(i) =>
                                              ::IResult::Incomplete(i),
                                              ::IResult::Done(i, o) => {
                                                  if i.input_len() ==
                                                         input.input_len() {
                                                      ::IResult::Error(::ErrorKind::SeparatedList)
                                                  } else {
                                                      res.push(o);
                                                      input = i;
                                                      let ret;
                                                      loop  {
                                                          let input_ =
                                                              input.clone();
                                                          match {
                                                                    use ::{Compare,
                                                                           CompareResult,
                                                                           InputLength,
                                                                           Slice};
                                                                    let res:
                                                                            ::IResult<_,
                                                                                      _> =
                                                                        match (input_).compare(" ")
                                                                            {
                                                                            CompareResult::Ok
                                                                            =>
                                                                            {
                                                                                let blen =
                                                                                    " ".input_len();
                                                                                ::IResult::Done(input_.slice(blen..),
                                                                                                input_.slice(..blen))
                                                                            }
                                                                            CompareResult::Incomplete
                                                                            =>
                                                                            {
                                                                                ::IResult::Incomplete(::Needed::Size(" ".input_len()))
                                                                            }
                                                                            CompareResult::Error
                                                                            =>
                                                                            {
                                                                                ::IResult::Error(::ErrorKind::Tag)
                                                                            }
                                                                        };
                                                                    res
                                                                } {
                                                              ::IResult::Error(_)
                                                              => {
                                                                  ret =
                                                                      ::IResult::Done(input,
                                                                                      res);
                                                                  break ;
                                                              }
                                                              ::IResult::Incomplete(::Needed::Unknown)
                                                              => {
                                                                  ret =
                                                                      ::IResult::Incomplete(::Needed::Unknown);
                                                                  break ;
                                                              }
                                                              ::IResult::Incomplete(::Needed::Size(needed))
                                                              => {
                                                                  let (size,
                                                                       overflowed) =
                                                                      needed.overflowing_add((i_).input_len()
                                                                                                 -
                                                                                                 input.input_len());
                                                                  ret =
                                                                      match overflowed
                                                                          {
                                                                          true
                                                                          =>
                                                                          ::IResult::Incomplete(::Needed::Unknown),
                                                                          false
                                                                          =>
                                                                          ::IResult::Incomplete(::Needed::Size(size)),
                                                                      };
                                                                  break ;
                                                              }
                                                              ::IResult::Done(i2,
                                                                              _)
                                                              => {
                                                                  let i2_len =
                                                                      i2.input_len();
                                                                  if i2_len ==
                                                                         input.input_len()
                                                                     {
                                                                      ret =
                                                                          ::IResult::Done(input,
                                                                                          res);
                                                                      break ;
                                                                  }
                                                                  match alphanumeric_or_bool(i2)
                                                                      {
                                                                      ::IResult::Error(_)
                                                                      => {
                                                                          ret
                                                                              =
                                                                              ::IResult::Done(input,
                                                                                              res);
                                                                          break
                                                                              ;
                                                                      }
                                                                      ::IResult::Incomplete(::Needed::Unknown)
                                                                      => {
                                                                          ret
                                                                              =
                                                                              ::IResult::Incomplete(::Needed::Unknown);
                                                                          break
                                                                              ;
                                                                      }
                                                                      ::IResult::Incomplete(::Needed::Size(needed))
                                                                      => {
                                                                          let (size,
                                                                               overflowed) =
                                                                              needed.overflowing_add((i_).input_len()
                                                                                                         -
                                                                                                         i2_len);
                                                                          ret
                                                                              =
                                                                              match overflowed
                                                                                  {
                                                                                  true
                                                                                  =>
                                                                                  ::IResult::Incomplete(::Needed::Unknown),
                                                                                  false
                                                                                  =>
                                                                                  ::IResult::Incomplete(::Needed::Size(size)),
                                                                              };
                                                                          break
                                                                              ;
                                                                      }
                                                                      ::IResult::Done(i3,
                                                                                      o3)
                                                                      => {
                                                                          if i3.input_len()
                                                                                 ==
                                                                                 i2_len
                                                                             {
                                                                              ret
                                                                                  =
                                                                                  ::IResult::Done(input,
                                                                                                  res);
                                                                              break
                                                                                  ;
                                                                          }
                                                                          res.push(o3);
                                                                          input
                                                                              =
                                                                              i3;
                                                                      }
                                                                  }
                                                              }
                                                          }
                                                      }
                                                      ret
                                                  }
                                              }
                                          }
                                      } {
                                    ::IResult::Error(e) =>
                                    ::IResult::Error(e),
                                    ::IResult::Incomplete(::Needed::Unknown)
                                    =>
                                    ::IResult::Incomplete(::Needed::Unknown),
                                    ::IResult::Incomplete(::Needed::Size(i))
                                    => {
                                        let (needed, overflowed) =
                                            (0usize +
                                                 (::InputLength::input_len(&(i))
                                                      -
                                                      ::InputLength::input_len(&i))).overflowing_add(i);
                                        match overflowed {
                                            true =>
                                            ::IResult::Incomplete(::Needed::Unknown),
                                            false =>
                                            ::IResult::Incomplete(::Needed::Size(needed)),
                                        }
                                    }
                                    ::IResult::Done(i, o) => {
                                        let args = o;
                                        let i_ = i.clone();
                                        ::IResult::Done(i_,
                                                        (ChurchValue::parse_function_application_to_church_value(function_name,
                                                                                                                 args)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        pub fn read_expr(input: &str) -> String {
            match church_parse(input) {
                IResult::Done(_, out) => out.unwrap().to_string(),
                IResult::Error(_) => String::from("ERROR"),
                _ => String::from("ERROR"),
            }
        }
        pub fn eval(input: ChurchValue)
         -> Result<ChurchValue, ChurchEvalError> {
            match input {
                ChurchValue::List(data) => Ok(ChurchValue::List(data)),
                ChurchValue::Bool(data) => Ok(ChurchValue::Bool(data)),
                ChurchValue::Number(data) => Ok(ChurchValue::Number(data)),
                ChurchValue::Func(fn_name, args) => {
                    let func =
                        self::primatives::PRIMATIVES.get(fn_name.as_str());
                    match func {
                        Some(fun) => {
                            let arg1 = args[0].clone();
                            let arg2 = args[1].clone();
                            fun(arg1, arg2)
                        }
                        None =>
                        Err(ChurchEvalError::FunctionNotFound(fn_name)),
                    }
                }
            }
        }
        pub fn apply(fn_name: &str, args: Vec<ChurchValue>)
         -> Result<ChurchValue, ChurchEvalError> {
            let function = self::primatives::PRIMATIVES.get(fn_name);
            match function {
                Some(fun) => {
                    let arg1 = args[0].clone();
                    let arg2 = args[1].clone();
                    let out = fun(arg1, arg2);
                    out
                }
                None => {
                    Err(ChurchEvalError::FunctionNotFound(fn_name.to_owned()))
                }
            }
        }
    }
    pub mod error {
        use std::fmt::{Display, Formatter};
        use std::error::{Error};
        use super::utils::vec_to_string;
        use super::parser::ChurchValue;
        trait ChurchErrorTrait: Error { }
        pub enum ChurchParseError {
            IntParseError,
            BoolParseError,
            ListParseError,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ChurchParseError {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&ChurchParseError::IntParseError,) => {
                        let mut builder =
                            __arg_0.debug_tuple("IntParseError");
                        builder.finish()
                    }
                    (&ChurchParseError::BoolParseError,) => {
                        let mut builder =
                            __arg_0.debug_tuple("BoolParseError");
                        builder.finish()
                    }
                    (&ChurchParseError::ListParseError,) => {
                        let mut builder =
                            __arg_0.debug_tuple("ListParseError");
                        builder.finish()
                    }
                }
            }
        }
        pub enum ChurchError {
            ParseError(ChurchParseError),
            EvalError(ChurchEvalError),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ChurchError {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&ChurchError::ParseError(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("ParseError");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&ChurchError::EvalError(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("EvalError");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                }
            }
        }
        impl Error for ChurchParseError {
            fn description(&self) -> &str {
                match self {
                    &ChurchParseError::IntParseError =>
                    "Failed to parse value into integer",
                    &ChurchParseError::BoolParseError =>
                    "Failed to parse value into boolean",
                    &ChurchParseError::ListParseError =>
                    "Failed to parse values into list",
                }
            }
        }
        impl Display for ChurchParseError {
            fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1_formatted(&[""],
                                                                    &match (&self,)
                                                                         {
                                                                         (__arg0,)
                                                                         =>
                                                                         [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                      ::std::fmt::Display::fmt)],
                                                                     },
                                                                    &[::std::fmt::rt::v1::Argument{position:
                                                                                                       ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                   format:
                                                                                                       ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                          ' ',
                                                                                                                                      align:
                                                                                                                                          ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                      flags:
                                                                                                                                          0u32,
                                                                                                                                      precision:
                                                                                                                                          ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                      width:
                                                                                                                                          ::std::fmt::rt::v1::Count::Implied,},}]))
            }
        }
        #[allow(unknown_lints)]
        #[allow(unused_doc_comment)]
        pub enum ChurchEvalError {
            FunctionNotFound(String),
            ArgumentError(String, Box<Vec<ChurchValue>>),
            TypeError(String, String, String),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(unknown_lints)]
        #[allow(unused_doc_comment)]
        impl ::std::fmt::Debug for ChurchEvalError {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&ChurchEvalError::FunctionNotFound(ref __self_0),) => {
                        let mut builder =
                            __arg_0.debug_tuple("FunctionNotFound");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&ChurchEvalError::ArgumentError(ref __self_0,
                                                     ref __self_1),) => {
                        let mut builder =
                            __arg_0.debug_tuple("ArgumentError");
                        let _ = builder.field(&&(*__self_0));
                        let _ = builder.field(&&(*__self_1));
                        builder.finish()
                    }
                    (&ChurchEvalError::TypeError(ref __self_0, ref __self_1,
                                                 ref __self_2),) => {
                        let mut builder = __arg_0.debug_tuple("TypeError");
                        let _ = builder.field(&&(*__self_0));
                        let _ = builder.field(&&(*__self_1));
                        let _ = builder.field(&&(*__self_2));
                        builder.finish()
                    }
                }
            }
        }
        #[allow(unused)]
        #[allow(unknown_lints)]
        #[allow(unused_doc_comment)]
        impl ::std::fmt::Display for ChurchEvalError {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ChurchEvalError::FunctionNotFound(ref fn_name) => {
                        let display_fn =
                            |_, f: &mut ::std::fmt::Formatter|
                                {
                                    f.write_fmt(::std::fmt::Arguments::new_v1_formatted(&["[!] Error: Function \"",
                                                                                          "\" was not found.\n"],
                                                                                        &match (&&fn_name,)
                                                                                             {
                                                                                             (__arg0,)
                                                                                             =>
                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                          ::std::fmt::Display::fmt)],
                                                                                         },
                                                                                        &[::std::fmt::rt::v1::Argument{position:
                                                                                                                           ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                       format:
                                                                                                                           ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,},}]))
                                };
                        display_fn(self, fmt)
                    }
                    ChurchEvalError::ArgumentError(ref fn_name, ref args) => {
                        let display_fn =
                            |_, f: &mut ::std::fmt::Formatter|
                                {
                                    f.write_fmt(::std::fmt::Arguments::new_v1_formatted(&["[!] Error: Wrong arguments for function ",
                                                                                          ".\n\tRecieved: ",
                                                                                          "\n"],
                                                                                        &match (&&fn_name,
                                                                                                &vec_to_string(*args.clone(),
                                                                                                               ", "))
                                                                                             {
                                                                                             (__arg0,
                                                                                              __arg1)
                                                                                             =>
                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                          ::std::fmt::Display::fmt),
                                                                                              ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                          ::std::fmt::Display::fmt)],
                                                                                         },
                                                                                        &[::std::fmt::rt::v1::Argument{position:
                                                                                                                           ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                       format:
                                                                                                                           ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,},},
                                                                                          ::std::fmt::rt::v1::Argument{position:
                                                                                                                           ::std::fmt::rt::v1::Position::At(1usize),
                                                                                                                       format:
                                                                                                                           ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,},}]))
                                };
                        display_fn(self, fmt)
                    }
                    ChurchEvalError::TypeError(ref fn_name, ref expected,
                                               ref actual) => {
                        let display_fn =
                            |_, f: &mut ::std::fmt::Formatter|
                                {
                                    f.write_fmt(::std::fmt::Arguments::new_v1_formatted(&["[!] Error: Wrong type of argument for function ",
                                                                                          ".\n\tExpected: ",
                                                                                          "\n\tRecieved: "],
                                                                                        &match (&&fn_name,
                                                                                                &&expected,
                                                                                                &&actual)
                                                                                             {
                                                                                             (__arg0,
                                                                                              __arg1,
                                                                                              __arg2)
                                                                                             =>
                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                          ::std::fmt::Display::fmt),
                                                                                              ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                          ::std::fmt::Display::fmt),
                                                                                              ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                          ::std::fmt::Display::fmt)],
                                                                                         },
                                                                                        &[::std::fmt::rt::v1::Argument{position:
                                                                                                                           ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                       format:
                                                                                                                           ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,},},
                                                                                          ::std::fmt::rt::v1::Argument{position:
                                                                                                                           ::std::fmt::rt::v1::Position::At(1usize),
                                                                                                                       format:
                                                                                                                           ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,},},
                                                                                          ::std::fmt::rt::v1::Argument{position:
                                                                                                                           ::std::fmt::rt::v1::Position::At(2usize),
                                                                                                                       format:
                                                                                                                           ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                              ' ',
                                                                                                                                                          align:
                                                                                                                                                              ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                          flags:
                                                                                                                                                              0u32,
                                                                                                                                                          precision:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                          width:
                                                                                                                                                              ::std::fmt::rt::v1::Count::Implied,},}]))
                                };
                        display_fn(self, fmt)
                    }
                }
            }
        }
        #[allow(unused)]
        #[allow(unknown_lints)]
        #[allow(unused_doc_comment)]
        impl ::std::error::Error for ChurchEvalError {
            fn description(&self) -> &str {
                match *self {
                    ChurchEvalError::FunctionNotFound(ref fn_name) => {
                        "Function Not Found"
                    }
                    ChurchEvalError::ArgumentError(ref fn_name, ref args) => {
                        "Argument Error"
                    }
                    ChurchEvalError::TypeError(ref fn_name, ref expected,
                                               ref actual) => {
                        "Wrong type"
                    }
                }
            }
            fn cause(&self) -> Option<&::std::error::Error> {
                match *self {
                    ChurchEvalError::FunctionNotFound(ref fn_name) => { None }
                    ChurchEvalError::ArgumentError(ref fn_name, ref args) => {
                        None
                    }
                    ChurchEvalError::TypeError(ref fn_name, ref expected,
                                               ref actual) => {
                        None
                    }
                }
            }
        }
        unsafe impl ::std::marker::Sync for ChurchEvalError { }
    }
    pub mod utils {
        use std::str::FromStr;
        pub fn vec_ref_to_string<T: ToString>(v: &Vec<T>, delim: &str)
         -> String {
            let mut out_str = String::from_str("").unwrap();
            for i in 0..v.len() {
                out_str.push_str(&v[i].to_string());
                if i < v.len() - 1 { out_str.push_str(delim); }
            }
            out_str
        }
        pub fn vec_to_string<T: ToString>(v: Vec<T>, delim: &str) -> String {
            let mut out_str = String::from_str("").unwrap();
            for i in 0..v.len() {
                ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["",
                                                                       "\n"],
                                                                     &match (&out_str,)
                                                                          {
                                                                          (__arg0,)
                                                                          =>
                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                       ::std::fmt::Display::fmt)],
                                                                      },
                                                                     &[::std::fmt::rt::v1::Argument{position:
                                                                                                        ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                    format:
                                                                                                        ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                           ' ',
                                                                                                                                       align:
                                                                                                                                           ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                       flags:
                                                                                                                                           0u32,
                                                                                                                                       precision:
                                                                                                                                           ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                       width:
                                                                                                                                           ::std::fmt::rt::v1::Count::Implied,},}]));
                out_str.push_str(&v[i].to_string());
                if i < v.len() - 1 { out_str.push_str(delim); }
            }
            out_str
        }
    }
}
use church::parser::read_expr;
use church::parser::*;
fn main() {
    let mut input = String::new();
    match apply("-",
                <[_]>::into_vec(box
                                    [apply("^",
                                           <[_]>::into_vec(box
                                                               [ChurchValue::Number(2),
                                                                ChurchValue::Number(13)])).unwrap(),
                                     ChurchValue::Number(1)])) {
        Ok(val) =>
        ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["", "\n"],
                                                             &match (&val.to_string(),)
                                                                  {
                                                                  (__arg0,) =>
                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                               ::std::fmt::Display::fmt)],
                                                              },
                                                             &[::std::fmt::rt::v1::Argument{position:
                                                                                                ::std::fmt::rt::v1::Position::At(0usize),
                                                                                            format:
                                                                                                ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                   ' ',
                                                                                                                               align:
                                                                                                                                   ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                               flags:
                                                                                                                                   0u32,
                                                                                                                               precision:
                                                                                                                                   ::std::fmt::rt::v1::Count::Implied,
                                                                                                                               width:
                                                                                                                                   ::std::fmt::rt::v1::Count::Implied,},}])),
        Err(err) =>
        ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["", "\n"],
                                                             &match (&err,) {
                                                                  (__arg0,) =>
                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                               ::std::fmt::Display::fmt)],
                                                              },
                                                             &[::std::fmt::rt::v1::Argument{position:
                                                                                                ::std::fmt::rt::v1::Position::At(0usize),
                                                                                            format:
                                                                                                ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                   ' ',
                                                                                                                               align:
                                                                                                                                   ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                               flags:
                                                                                                                                   0u32,
                                                                                                                               precision:
                                                                                                                                   ::std::fmt::rt::v1::Count::Implied,
                                                                                                                               width:
                                                                                                                                   ::std::fmt::rt::v1::Count::Implied,},}])),
    }
    std::io::stdin().read_line(&mut input).ok().expect("FAILED");
    let res = read_expr(&input);
    ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["", "\n\n\n\n"],
                                                         &match (&res.to_string(),)
                                                              {
                                                              (__arg0,) =>
                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                           ::std::fmt::Display::fmt)],
                                                          },
                                                         &[::std::fmt::rt::v1::Argument{position:
                                                                                            ::std::fmt::rt::v1::Position::At(0usize),
                                                                                        format:
                                                                                            ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                               ' ',
                                                                                                                           align:
                                                                                                                               ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                           flags:
                                                                                                                               0u32,
                                                                                                                           precision:
                                                                                                                               ::std::fmt::rt::v1::Count::Implied,
                                                                                                                           width:
                                                                                                                               ::std::fmt::rt::v1::Count::Implied,},}]));
}
