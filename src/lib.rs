// Copyright 2023 jmjoy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(error_generic_member_access)]
#![feature(provide_any)]
#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

use std::{
    any::{type_name, Any, Demand, TypeId},
    backtrace::{Backtrace, BacktraceStatus},
    borrow::Cow,
    error,
    fmt::{self, Debug, Display},
    result,
};

struct Type {
    type_id: TypeId,
    type_name: &'static str,
}

impl Type {
    fn new<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
        }
    }
}

/// Alias of [Result](result::Result)<T, [Error]>;
pub type Result<T> = result::Result<T, Error>;

/// The only one [`Error`].
pub struct Error {
    r#type: Type,
    sub_type: Type,
    message: Cow<'static, str>,
    backtrace: Backtrace,
    data: Option<Box<dyn Any + Send + Sync>>,
    source: Option<Box<dyn error::Error + Send + Sync + 'static>>,
}

impl Error {
    /// Detect [Exception] is belong to the type.
    #[inline]
    pub fn is<T: 'static>(&self) -> bool {
        self.r#type.type_id == TypeId::of::<T>()
    }

    /// Detect [Error] is belong to the sub type.
    #[inline]
    pub fn is_sub<T: 'static>(&self) -> bool {
        self.sub_type.type_id == TypeId::of::<T>()
    }

    /// Get immutable reference data.
    pub fn data<D: 'static>(&self) -> Option<&D> {
        self.data
            .as_deref()
            .and_then(|data| (data as &dyn Any).downcast_ref())
    }

    /// Convert into data.
    pub fn into_data<D: 'static>(self) -> Option<D> {
        self.data
            .and_then(|data| data.downcast().ok())
            .map(|data| *data)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)?;

        if f.alternate() {
            write!(f, "\nType: {}", &self.r#type.type_name)?;
            if self.sub_type.type_id != TypeId::of::<()>() {
                write!(f, "\nSub type: {}", &self.sub_type.type_name)?;
            }
        }

        if self.backtrace.status() == BacktraceStatus::Captured {
            write!(f, "\nBacktrace:\n{}", &self.backtrace.to_string())?;
        }

        Ok(())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_deref().map(|s| s as _)
    }

    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        demand.provide_ref(&self.r#type);
        demand.provide_ref(&self.backtrace);
    }
}

/// Builder of [Error].
pub struct ErrorBuilder {
    r#type: Type,
    sub_type: Option<Type>,
    message: Option<Cow<'static, str>>,
    data: Option<Box<dyn Any + Send + Sync>>,
    source: Option<Box<dyn error::Error + Send + Sync + 'static>>,
}

impl ErrorBuilder {
    /// Create error builder.
    pub fn new<T: 'static>() -> Self {
        Self {
            r#type: Type::new::<T>(),
            sub_type: None,
            message: None,
            data: None,
            source: None,
        }
    }

    /// Set sub type.
    pub fn sub_type<T: 'static>(mut self) -> Self {
        self.sub_type = Some(Type::new::<T>());
        self
    }

    /// Set message.
    pub fn message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set data.
    pub fn data(mut self, data: impl Any + Send + Sync) -> Self {
        self.data = Some(Box::new(data));
        self
    }

    /// Set source.
    pub fn source(
        mut self, source: impl Into<Box<dyn error::Error + Send + Sync + 'static>>,
    ) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Build error.
    pub fn build(self) -> Error {
        Error {
            r#type: self.r#type,
            sub_type: self.sub_type.unwrap_or(Type::new::<()>()),
            message: self.message.unwrap_or_default(),
            backtrace: Backtrace::capture(),
            data: self.data,
            source: self.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait AssertSendSync: Send + Sync + 'static {}

    impl AssertSendSync for Error {}

    #[test]
    fn test_exception() {
        pub struct FooError;
        let err = ErrorBuilder::new::<FooError>().message("foo error").build();
        assert!(err.is::<FooError>());
        assert!(err.is_sub::<()>());
        assert_eq!(err.to_string(), "foo error");

        dbg!(err);
    }
}
