use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

/// Wrap value in Box.
pub fn to_box<T>(value: T) -> Box<T> {
    Box::new(value)
}

/// Wrap value in Rc.
pub fn to_rc<T>(value: T) -> Rc<T> {
    Rc::new(value)
}

/// Wrap value in Arc.
pub fn to_arc<T>(value: T) -> Arc<T> {
    Arc::new(value)
}

/// Convert Box<T> to T (unbox).
pub fn unbox<T>(boxed: Box<T>) -> T {
    *boxed
}

/// Clone from Rc<T>.
pub fn rc_to_owned<T: Clone>(rc: &Rc<T>) -> T {
    (**rc).clone()
}

/// Clone from Arc<T>.
pub fn arc_to_owned<T: Clone>(arc: &Arc<T>) -> T {
    (**arc).clone()
}

/// Convert &str to Cow<str>.
pub fn str_to_cow(s: &str) -> Cow<'_, str> {
    Cow::Borrowed(s)
}

/// Convert String to Cow<str>.
pub fn string_to_cow(s: String) -> Cow<'static, str> {
    Cow::Owned(s)
}

/// Convert Cow<str> to String.
pub fn cow_to_string(cow: Cow<'_, str>) -> String {
    cow.into_owned()
}

/// Convert &[T] to Cow<[T]>.
pub fn slice_to_cow<T: Clone>(s: &[T]) -> Cow<'_, [T]> {
    Cow::Borrowed(s)
}

/// Convert Vec<T> to Cow<[T]>.
pub fn vec_to_cow<T: Clone>(v: Vec<T>) -> Cow<'static, [T]> {
    Cow::Owned(v)
}
