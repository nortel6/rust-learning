#[macro_export]
// vec! macro, although simpler
macro_rules! vec {
    // Pattern matching...
    // expression named $x
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // For each match
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

