macro_rules! size_of {
($($t:ty), *) => {{
    let mut length = 0;
    $(length += ::std::mem::size_of::<$t>();); *
    length as usize
}};
}

macro_rules! enum_serialize {
    ($e:ty => {$( $t:path => $s:expr ),* }) => {
impl ::serde::Serialize for $e {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match *self {
            $($t => $s,)*
        };

        serializer.serialize_str(value)
    }
}


impl<'d> ::serde::Deserialize<'d> for $e {

    fn deserialize<D>(deserializer: D) -> Result<$e, D::Error>
    where
        D: Deserializer<'d>,
    {
        struct EnumVisitor;

        impl<'d> Visitor<'d> for EnumVisitor {
            type Value = $e;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an string representation of enum")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                match value {
                    $($s => Ok($t),) *
                    _ => Err(E::custom(format!("Unsupported enum value: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(EnumVisitor)
    }
}

};
}
