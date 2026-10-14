// Defines a fieldless enum whose variants correspond to the string values Kubernetes uses for phases, states, condition types, etc.
//
// Invocations of this macro are emitted by the codegen for string fields like `PodStatus::phase`, whose values are one of a fixed set
// but which the upstream spec types as plain strings. See `k8s-openapi-codegen/src/string_enums.rs`.
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            /// All variants of this enum, in declaration order.
            pub const VARIANTS: &'static [Self] = &[$(Self::$variant,)+];

            /// Returns the string value of this variant.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }

            /// Returns the variant corresponding to the given string value, or `None` if it is not recognized.
            pub fn parse(s: &str) -> ::core::option::Option<Self> {
                match s {
                    $($value => ::core::option::Option::Some(Self::$variant),)+
                    _ => ::core::option::Option::None,
                }
            }
        }

        impl ::core::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = $crate::UnknownStringEnumValue;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                Self::parse(s).ok_or_else(|| $crate::UnknownStringEnumValue::new(::core::stringify!($name), s))
            }
        }

        impl ::core::convert::TryFrom<&str> for $name {
            type Error = $crate::UnknownStringEnumValue;

            fn try_from(s: &str) -> ::core::result::Result<Self, Self::Error> {
                ::core::str::FromStr::from_str(s)
            }
        }

        impl ::core::cmp::PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl ::core::cmp::PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl ::core::cmp::PartialEq<std::string::String> for $name {
            fn eq(&self, other: &std::string::String) -> bool {
                self.as_str() == other
            }
        }

        impl ::core::cmp::PartialEq<$name> for str {
            fn eq(&self, other: &$name) -> bool {
                self == other.as_str()
            }
        }

        impl ::core::cmp::PartialEq<$name> for &str {
            fn eq(&self, other: &$name) -> bool {
                *self == other.as_str()
            }
        }

        impl ::core::cmp::PartialEq<$name> for std::string::String {
            fn eq(&self, other: &$name) -> bool {
                self == other.as_str()
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error> where D: $crate::serde::Deserializer<'de> {
                struct Visitor;

                impl $crate::serde::de::Visitor<'_> for Visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.write_str(::core::concat!("a ", ::core::stringify!($name), " string"))
                    }

                    fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E> where E: $crate::serde::de::Error {
                        $name::parse(v).ok_or_else(|| $crate::serde::de::Error::unknown_variant(v, &[$($value,)+]))
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }

        impl $crate::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> where S: $crate::serde::Serializer {
                serializer.serialize_str(self.as_str())
            }
        }
    };
}
pub(crate) use string_enum;

/// The error returned when parsing a string into a string enum like [`PodPhase`](crate::api::core::v1::PodPhase) fails because the string is not one of its values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownStringEnumValue {
    type_name: &'static str,
    value: std::string::String,
}

impl UnknownStringEnumValue {
    #[doc(hidden)]
    pub fn new(type_name: &'static str, value: &str) -> Self {
        UnknownStringEnumValue { type_name, value: value.into() }
    }

    /// The name of the enum type that the string was being parsed into.
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    /// The string that was not recognized.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for UnknownStringEnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown {} value {:?}", self.type_name, self.value)
    }
}

impl core::error::Error for UnknownStringEnumValue {}
