#[macro_export]
macro_rules! iif {
    ( $check:expr, $true_case:expr, $false_case:expr ) => {
        if $check { $true_case } else { $false_case }
    };
}

#[macro_export]
macro_rules! iif_opt {
    ( $check:expr, $true_case:expr ) => {
        if $check { Some($true_case) } else { None }
    };
}
