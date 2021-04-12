#[macro_export]
macro_rules! iif {
    ( $check:expr, $true_case:expr, $false_case:expr ) => {
        if $check { $true_case } else { $false_case }
    };
}
