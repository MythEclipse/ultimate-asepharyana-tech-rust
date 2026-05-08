/// Convert Option<T> to Result<T, E>.
pub fn option_to_result<T, E>(opt: Option<T>, err: E) -> Result<T, E> {
    opt.ok_or(err)
}

/// Convert Option<T> to Result<T, String>.
pub fn option_to_result_str<T>(opt: Option<T>, msg: &str) -> Result<T, String> {
    opt.ok_or_else(|| msg.to_string())
}

/// Convert Result<T, E> to Option<T>.
pub fn result_to_option<T, E>(res: Result<T, E>) -> Option<T> {
    res.ok()
}

/// Convert Result<T, E> to Option<E>.
pub fn result_to_err<T, E>(res: Result<T, E>) -> Option<E> {
    res.err()
}

/// Flatten nested Option.
pub fn flatten_option<T>(opt: Option<Option<T>>) -> Option<T> {
    opt.flatten()
}

/// Flatten nested Result.
pub fn flatten_result<T, E>(res: Result<Result<T, E>, E>) -> Result<T, E> {
    res.and_then(|r| r)
}

/// Transpose Option<Result<T, E>> to Result<Option<T>, E>.
pub fn transpose_option_result<T, E>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    opt.transpose()
}

/// Transpose Result<Option<T>, E> to Option<Result<T, E>>.
pub fn transpose_result_option<T, E>(res: Result<Option<T>, E>) -> Option<Result<T, E>> {
    res.transpose()
}
