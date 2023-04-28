#[macro_export]
macro_rules! my_vec {
    ($($x: expr), *) => {
        {
            let mut temp_v = Vec::new();
            $(
                temp_v.push($x);
            )*
            temp_v
        }
    };
}
