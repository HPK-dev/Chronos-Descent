///Define a wrapper
#[macro_export]
macro_rules! define_mapping {
    ($(
        $( #[$meta:meta] )*
        $name:ident => ( $inner:ty ) ;
    )*) => {
        $(
        $( #[$meta] )*
        pub struct $name (pub $inner);

        impl std::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<$inner> for $name {
            fn from(inner: $inner) -> Self {
                Self(inner)
            }
        }


    )*};
}

#[macro_export]
macro_rules! unwrap_or {
    ( $default:expr $(,)? ) => {
        $default
    };
    ( $default:expr, $value:expr) => {
        $value
    };
}
