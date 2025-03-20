use piccolo::{FromValue, Value};

/// Trait representing any kind of permission, with varying degrees
///
/// [`Permission`] requires the implementor to define two levels of granted permissions:
/// - [`all`]: As much permissions as the implementor can represent
/// - [`none`]: Absolutely zero permission
pub trait Permission: Eq + Sized {
    /// Construct a [`Permission`] requesting as much as the implementor can represent
    fn all() -> Self;

    /// Construct a [`Permission`] requesting absolutely zero permission
    fn none() -> Self;

    fn is_all(&self) -> bool {
        *self == Self::all()
    }

    fn is_none(&self) -> bool {
        *self == Self::none()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ReadWritePermissions {
    pub read: bool,
    pub write: bool,
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;
    use std::f64::NAN;
    #[derive(Debug, PartialEq, strict_partial_ord_derive::PartialOrd)]
    struct Point3 {
        x: f32,
        y: f32,
        z: f32,
    }

    impl From<(f32, f32, f32)> for Point3 {
        fn from((x, y, z): (f32, f32, f32)) -> Self {
            Self { x, y, z }
        }
    }

    // Point3 PartialCmp Test function
    fn p3_pcmp_test<T: Into<Point3>>(p1: T, p2: T, expected: Option<Ordering>) {
        assert_eq!(p1.into().partial_cmp(&p2.into()), expected);
    }

    #[test]
    fn basic_equality() {
        p3_pcmp_test((0., 0., 0.), (0., 0., 0.), Some(Equal));
    }

    #[test]
    fn lesser() {
        p3_pcmp_test((0., 0., 0.), (1., 1., 1.), Some(Less));
        p3_pcmp_test((0., 1., 2.), (1., 2., 3.), Some(Less));
    }

    #[test]
    fn greater() {
        p3_pcmp_test((1., 1., 1.), (0., 0., 0.), Some(Greater));
        p3_pcmp_test((1., 2., 3.), (0., 1., 2.), Some(Greater));
    }

    #[test]
    fn not_comparable() {
        p3_pcmp_test((0., 0., 0.), (-1., 0., 1.), None);
        p3_pcmp_test((-1., 0., 1.), (0., 0., 0.), None);
        p3_pcmp_test((0., 0., 0.), (0., NAN as f32, 0.), None);
    }
}

impl Permission for ReadWritePermissions {
    fn all() -> Self {
        Self {
            read: true,
            write: true,
        }
    }

    fn none() -> Self {
        Self::default()
    }
}

impl<'gc> FromValue<'gc> for ReadWritePermissions {
    fn from_value(
        _: piccolo::Context<'gc>,
        value: piccolo::Value<'gc>,
    ) -> Result<Self, piccolo::TypeError> {
        match value {
            Value::Nil | Value::Boolean(false) => Ok(Self::none()),
            Value::Boolean(true) => Ok(Self::all()),
            Value::String(s) => match s.as_bytes() {
                b"all" => Ok(Self::all()),
                b"none" => Ok(Self::none()),
                _ => Err(piccolo::TypeError {
                    expected: "all / none",
                    found: "",
                }),
            },
            Value::Table(tab) => {
                let mut perms = Self::none();

                for (key, val) in tab {
                    let (Value::Integer(_), Value::String(s)) = (key, val) else {
                        eprintln!(
                            "Warn: skipping KV pair in read/write permissions: ([{key}] = {val})"
                        );
                        continue;
                    };
                    match s.as_bytes() {
                        b"read" => perms.read = true,
                        b"write" => perms.write = true,
                        _ => eprintln!("Warn: skipping unknown perm request: {s}"),
                    }
                }
                Ok(perms)
            }
            _ => Err(piccolo::TypeError {
                expected: "permissions table",
                found: value.type_name(),
            }),
        }
    }
}
