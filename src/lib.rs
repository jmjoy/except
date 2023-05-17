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

#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

use std::{
    any::{type_name, Any, TypeId},
    error::Error,
    fmt::{self, Debug, Display}, backtrace::{Backtrace, BacktraceStatus}
};

/// Exception: The only `Error` you deserve.
pub struct Exception {
    kind_id: TypeId,
    kind_name: &'static str,
    code: u32,
    message: String,
    backtrace: Backtrace,
    data: Option<Box<dyn Any + Send + Sync>>,
    source: Option<Box<Exception>>,
}

impl Exception {
    /// New [Exception] with kind and message.
    pub fn new<T: 'static>(message: impl Into<String>) -> Self {
        Self {
            kind_id: TypeId::of::<T>(),
            kind_name: type_name::<T>(),
            code: 0,
            message: message.into(),
            backtrace: Backtrace::capture(),
            data: None,
            source: None,
        }
    }

    /// New [Exception] with kind, code and message.
    pub fn new1<T: 'static>(message: impl Into<String>, code: u32) -> Self {
        Self {
            kind_id: TypeId::of::<T>(),
            kind_name: type_name::<T>(),
            code,
            message: message.into(),
            backtrace: Backtrace::capture(),
            data: None,
            source: None,
        }
    }

    /// New [Exception] with kind, code, message and data.
    pub fn new2<T: 'static>(message: impl Into<String>, code: u32, data: impl Any + Send + Sync) -> Self {
        Self {
            kind_id: TypeId::of::<T>(),
            kind_name: type_name::<T>(),
            code,
            message: message.into(),
            backtrace: Backtrace::capture(),
            data: Some(Box::new(data)),
            source: None,
        }
    }

    /// New [Exception] with kind, code, message and source.
    pub fn new3<T: 'static>(message: impl Into<String>, code: u32, source: Exception) -> Self {
        Self {
            kind_id: TypeId::of::<T>(),
            kind_name: type_name::<T>(),
            code,
            message: message.into(),
            backtrace: Backtrace::capture(),
            data: None,
            source: Some(Box::new(source)),
        }
    }

    /// New [Exception] with kind, code, message, data and source.
    pub fn new4<T: 'static>(
        message: impl Into<String>, code: u32, data: impl Any + Send + Sync, source: Exception    ) -> Self {
        Self {
                kind_id: TypeId::of::<T>(),
                kind_name: type_name::<T>(),
            code,
                message: message.into(),
                backtrace: Backtrace::capture(),
                data: Some(Box::new(data)),
                source: Some(Box::new(source)),
        }
    }

    /// Detect [Exception] is belong to the kind.
    pub fn is<T: 'static>(&self) -> bool {
        self.kind_id == TypeId::of::<T>()
    }

    /// Get kind name.
    pub fn get_kind_name(&self) -> &'static str {
        self.kind_name
    }

    /// Get code.
    pub fn get_code(&self) -> u32 {
        self.code
    }

    /// Get source exception.
    pub fn get_source(&self) -> Option<&Self> {
        self.source.as_deref()
    }

    /// Get backtrace.
    pub fn get_backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    /// Get immutable reference data.
    pub fn get_data<D: 'static>(&self) -> Option<&D> {
        self .data
            .as_deref()
            .and_then(|data| (data as &dyn Any).downcast_ref())
    }

    /// Convert into data.
    pub fn into_data<D: 'static>(self) -> Option<D> {
        self .data
            .and_then(|data| data.downcast().ok())
            .map(|data| *data)
    }
}

impl Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Exception")
            .field("kind_name", &self.kind_name)
            .field("message", &self.message)
            .field("backtrace", &self.backtrace)
            .finish()
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)?;
        if self.backtrace.status() == BacktraceStatus::Captured {
            write!(f, "\n{}", &self.backtrace)?;
        }
        Ok(())
    }
}

impl<E: Error + Send + Sync + 'static> From<E> for Exception {
    fn from(e: E) -> Self {
        Self::new2::<E>(e.to_string(), 0, e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait AssertSendSync: Send + Sync + 'static {}

    impl AssertSendSync for Exception {}

    #[test]
    fn test_exception_is() {
        pub struct FooError;
        let e = Exception::new::<FooError>("foo error");
        assert!(e.is::<FooError>());
    }

    #[test]
    fn test_exception_get_data() {
        let e = Exception::new2::<()>("foo error", 0, 100usize);
        assert_eq!(e.get_data::<usize>(), Some(&100usize));
    }
}
