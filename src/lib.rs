//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/result-rs/")]

//! Helpers for dealing with `Result` and `Option` types.

/// Extension trait for nested `Option` and `Result` types.
pub trait OptionResultExt {
    type Out;

    /// Inverts a nested `Option<Result<T, E>>` or `Result<Option<T>, E>`
    fn invert(self) -> Self::Out;
}

impl<T, E> OptionResultExt for Option<Result<T, E>> {
    type Out = Result<Option<T>, E>;

    fn invert(self) -> Self::Out {
        match self {
            Some(Err(e)) => Err(e),
            Some(Ok(v)) => Ok(Some(v)),
            None => Ok(None)
        }
    }
}

impl<T, E> OptionResultExt for Result<Option<T>, E> {
    type Out = Option<Result<T, E>>;

    fn invert(self) -> Self::Out {
        match self {
            Ok(None) => None,
            Ok(Some(v)) => Some(Ok(v)),
            Err(e) => Some(Err(e)),
        }
    }
}
