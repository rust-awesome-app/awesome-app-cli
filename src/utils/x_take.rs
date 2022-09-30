use crate::prelude::{Error, Result};

// region:    --- XTake

/// Remove and return the Option<value> for a given type and key.
/// If no value for this key, return Result<None>.
/// If type missmatch, return a Error.
pub trait XTakeImpl<T> {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<T>>;
}

/// For turbofish friendly version of XTakeInto with blanket implementation.
/// Note: Has a blanket implementation. Not to be implemented directly.
///       XTakeInto is the to be implemented trait
pub trait XTake {
	fn x_take<T>(&mut self, k: &str) -> Result<Option<T>>
	where
		Self: XTakeImpl<T>;
}

/// Blanket implementation
impl<S> XTake for S {
	fn x_take<T>(&mut self, k: &str) -> Result<Option<T>>
	where
		S: XTakeImpl<T>,
	{
		XTakeImpl::x_take_impl(self, k)
	}
}

/// Take the value and return Error if None.
/// Note: Has a blanket implementation. Not to be implemented directly.
///       XTakeInto is the to be implemented trait
pub trait XTakeVal {
	fn x_take_val<T>(&mut self, k: &str) -> Result<T>
	where
		Self: XTakeImpl<T>;
}

/// Blanket implementation
impl<S> XTakeVal for S {
	fn x_take_val<T>(&mut self, k: &str) -> Result<T>
	where
		S: XTakeImpl<T>,
	{
		let val: Option<T> = XTakeImpl::x_take_impl(self, k)?;
		val.ok_or_else(|| Error::XtakePropNotFound(k.to_string()))
	}
}

// endregion: --- XTake