#[macro_export]
/// Returns one of two parts, depending on the evaluation of an expression.
/// Example:
/// ```rust
/// use iif::iif;
///
/// let active = true;
/// let active_text = iif!(active, "Active", "Inactive");
///
/// assert_eq!(active_text, "Active");
///
/// let active = false;
/// let active_text = iif!(active, "Active", "Inactive");
///
/// assert_eq!(active_text, "Inactive");
/// ```
macro_rules! iif {
    ( $check:expr, $true_case:expr, $false_case:expr ) => {
        if $check { $true_case } else { $false_case }
    };
}

#[macro_export]
/// Returns an option, depending on the evaluation of an expression.
/// Example:
/// ```rust
/// use iif::iif_opt;
///
/// let active = true;
/// let active_text = iif_opt!(active, "Active");
///
/// assert_eq!(active_text, Some("Active"));
///
/// let active = false;
/// let active_text = iif_opt!(active, "Active");
///
/// assert_eq!(active_text, None);
/// ```
macro_rules! iif_opt {
    ( $check:expr, $true_case:expr ) => {
        if $check { Some($true_case) } else { None }
    };
}
