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
    fmt::{self, Debug, Display},
    sync::Arc,
};

struct InnerException {
    kind_id: TypeId,
    kind: &'static str,
    message: String,
    data: Option<Box<dyn Any + Send + Sync>>,
    source: Option<Exception>,
}

/// The only `Error` you deserve.
#[derive(Clone)]
pub struct Exception {
    inner: Arc<InnerException>,
}

impl Exception {
    /// New [Exception] with kind and message.
    pub fn new<T: 'static>(message: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(InnerException {
                kind_id: TypeId::of::<T>(),
                kind: type_name::<T>(),
                message: message.into(),
                data: None,
                source: None,
            }),
        }
    }

    /// New [Exception] with kind, message and data.
    pub fn new2<T: 'static>(message: impl Into<String>, data: impl Any + Send + Sync) -> Self {
        Self {
            inner: Arc::new(InnerException {
                kind_id: TypeId::of::<T>(),
                kind: type_name::<T>(),
                message: message.into(),
                data: Some(Box::new(data)),
                source: None,
            }),
        }
    }

    /// New [Exception] with kind, message and source.
    pub fn new3<T: 'static>(message: impl Into<String>, source: &Exception) -> Self {
        Self {
            inner: Arc::new(InnerException {
                kind_id: TypeId::of::<T>(),
                kind: type_name::<T>(),
                message: message.into(),
                data: None,
                source: Some(source.clone()),
            }),
        }
    }

    /// New [Exception] with kind, message, data and source.
    pub fn new4<T: 'static>(
        message: impl Into<String>, data: impl Any + Send + Sync, source: &Exception,
    ) -> Self {
        Self {
            inner: Arc::new(InnerException {
                kind_id: TypeId::of::<T>(),
                kind: type_name::<T>(),
                message: message.into(),
                data: Some(Box::new(data)),
                source: Some(source.clone()),
            }),
        }
    }

    /// Detect [Exception] is belong to the kind.
    pub fn is<T: 'static>(&self) -> bool {
        self.inner.kind_id == TypeId::of::<T>()
    }

    /// Get immutable reference data.
    pub fn data<D: 'static>(&self) -> Option<&D> {
        self.inner
            .data
            .as_deref()
            .and_then(|data| (data as &dyn Any).downcast_ref())
    }
}

impl Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Exception")
            .field("kind", &self.inner.kind)
            .field("message", &self.inner.message)
            .finish()
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner.message, f)
    }
}

impl Error for Exception {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.source.as_ref().map(|s| s as _)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exception_is() {
        pub struct FooError;
        let e = Exception::new::<FooError>("foo error");
        assert!(e.is::<FooError>());
    }

    #[test]
    fn test_exception_data() {
        let e = Exception::new2::<()>("foo error", 100usize);
        assert_eq!(e.data::<usize>(), Some(&100usize));
    }
}
