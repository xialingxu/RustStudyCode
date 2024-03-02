#[macro_export]
macro_rules! my_macro {
    ($($x:expr),*) => {
        println!($($x),*);
    };
}