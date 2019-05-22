#[macro_export]
macro_rules! mice {
    ($(($input:expr, $expected:expr, $name:ident))*, $function:ident) => {

       $(
        #[test]
        fn $name() {
            let output = $function($input);
            assert_eq!(output, $expected);

        }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    mice![("foo", "foo", test_foo_bar_baz), f];

    fn f(a: &str) -> &str {
        a
    }
}
