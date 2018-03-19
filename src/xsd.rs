use document::{Iri, Literal};

pub trait Xsd {
    const DATATYPE: Iri;
}

macro_rules! xsd_literal_from_parse {
    ($($t:ty, $x:tt),* $(,)*) => {
        $(
            impl From<$t> for Literal {
                fn from(t: $t) -> Self {
                    let ty = iri!($x);
                    Literal::typed(t.to_string().as_str(), ty)
                }
            }

            impl Xsd for $t {
                const DATATYPE: Iri = iri!($x);
            }
        )*
    };
}

xsd_literal_from_parse! {
    i8, "http://www.w3.org/2001/XMLSchema#byte",
    i16, "http://www.w3.org/2001/XMLSchema#short",
    i32, "http://www.w3.org/2001/XMLSchema#int",
    i64, "http://www.w3.org/2001/XMLSchema#long",

    u8, "http://www.w3.org/2001/XMLSchema#unsignedByte",
    u16, "http://www.w3.org/2001/XMLSchema#unsignedShort",
    u32, "http://www.w3.org/2001/XMLSchema#unsignedInt",
    u64, "http://www.w3.org/2001/XMLSchema#unsignedLong",
}
