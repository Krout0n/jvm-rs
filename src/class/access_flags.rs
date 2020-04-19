use super::common_type::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AccessFlagKind {
    PUBLIC,
    PRIVATE,
    PROTECTED,
    STATIC,
    FINAL,
    SYNCHRONIZED,
    BRIDGE,
    VARARGS,
    NATIVE,
    ABSTRACT,
    STRICT,
    SYNTHETIC,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccessFlags(Vec<AccessFlagKind>);

impl From<u2> for AccessFlags {
    fn from(n: u2) -> Self {
        use AccessFlagKind::*;
        let mut v = vec![];
        let mut n = n;
        macro_rules! push {
            ($value: expr, $kind: ident) => {
                if n >= $value {
                    v.push($kind);
                    n -= $value;
                }
            };
        }
        push!(0x1000, SYNTHETIC);
        push!(0x0800, STRICT);
        push!(0x0400, ABSTRACT);
        push!(0x0100, NATIVE);
        push!(0x0080, VARARGS);
        push!(0x0040, BRIDGE);
        push!(0x0020, SYNCHRONIZED);
        push!(0x0010, FINAL);
        push!(0x0008, STATIC);
        push!(0x0004, PROTECTED);
        push!(0x0002, PRIVATE);
        push!(0x0001, PUBLIC);
        Self(v)
    }
}
