#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    compat::{error::Error, fmt},
    value_type::ValueType,
};

/// An error returned by a client-provided external function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalFunctionError(String);

impl ExternalFunctionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for ExternalFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for ExternalFunctionError {}

/// The result of an external function call. `None` represents Ink's void value.
pub type ExternalFunctionResult = Result<Option<ValueType>, ExternalFunctionError>;

/// An external call that could not be answered synchronously and is waiting for
/// the host to call [`Story::resume_external`](crate::story::Story::resume_external).
#[derive(Clone, PartialEq)]
pub struct PendingExternal {
    name: String,
    args: Vec<ValueType>,
}

impl PendingExternal {
    pub(crate) fn new(name: String, args: Vec<ValueType>) -> Self {
        Self { name, args }
    }

    /// The Ink `EXTERNAL` name that is waiting.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The arguments passed by Ink, in source order.
    pub fn args(&self) -> &[ValueType] {
        &self.args
    }
}

/// The outcome of a resumable external function.
#[derive(Clone, PartialEq)]
pub enum ExternalOutcome {
    /// The call finished immediately. `None` is Ink's void value.
    Done(Option<ValueType>),
    /// The call needs host state that is not available yet. The story stops at
    /// this call until the host resumes it.
    Pending,
}

/// The result of a resumable external function call.
pub type PendingExternalFunctionResult = Result<ExternalOutcome, ExternalFunctionError>;

/// Defines a callback that may suspend the story and be resumed later.
pub trait PendingExternalFunction {
    fn call(&mut self, func_name: &str, args: &[ValueType]) -> PendingExternalFunctionResult;
}

impl<F> PendingExternalFunction for F
where
    F: FnMut(&str, &[ValueType]) -> PendingExternalFunctionResult,
{
    fn call(&mut self, func_name: &str, args: &[ValueType]) -> PendingExternalFunctionResult {
        self(func_name, args)
    }
}

/// Defines a callback implementing an external Ink function.
pub trait ExternalFunction {
    fn call(&mut self, func_name: &str, args: &[ValueType]) -> ExternalFunctionResult;
}

impl<F> ExternalFunction for F
where
    F: FnMut(&str, &[ValueType]) -> ExternalFunctionResult,
{
    fn call(&mut self, func_name: &str, args: &[ValueType]) -> ExternalFunctionResult {
        self(func_name, args)
    }
}
