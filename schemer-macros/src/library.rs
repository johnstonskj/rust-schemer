#[macro_export]
macro_rules! library_name {
    ($name:ident, $value:expr, $fn_name:ident) => {
        pub const $name: &str = $value;

        pub fn $fn_name() -> LibraryName {
            LibraryName::new_unchecked(&[$name])
        }
    };
    ($name:ident, $value:expr, $parent:ident, $fn_name:ident) => {
        pub const $name: &str = $value;

        pub fn $fn_name() -> LibraryName {
            LibraryName::new_unchecked(&[$parent, $name])
        }
    };
}
