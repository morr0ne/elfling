pub mod header;
pub mod program_header;

macro_rules! elf_enum {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($inner_type:ty) {
            $(
                $(#[doc = $doc:expr])*
                $const_name:ident = $value:expr, $human_string:expr,
            )*
        }
    ) => {
        #[repr(transparent)]
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
        $(#[$attr])*
        $vis struct $name {
            inner: $inner_type,
        }

        impl $name {
            $(
                $(#[doc = $doc])*
                pub const $const_name: Self = Self::from_raw($value);
            )*

            pub const fn as_human_string(&self) -> &'static str {
                match *self {
                    $(
                        #[allow(unreachable_patterns)]
                        Self::$const_name => $human_string,
                    )*
                    _ => "unknown"
                }
            }

            pub const fn from_raw(raw: $inner_type) -> Self {
                Self { inner: raw }
            }

            pub const fn into_raw(self) -> $inner_type {
                self.inner
            }

            pub const fn as_raw(&self) -> $inner_type {
                self.inner
            }

            pub(crate) const fn debug_name(&self) -> &'static str {
                match *self {
                    $(
                        #[allow(unreachable_patterns)]
                        Self::$const_name => stringify!($const_name),
                    )*
                    _ => "UNKNOWN"
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", self.debug_name(), self.inner)
            }
        }

        impl From<$name> for $inner_type {
            fn from(e: $name) -> $inner_type {
                e.into_raw()
            }
        }
    };
}

pub(crate) use elf_enum;
