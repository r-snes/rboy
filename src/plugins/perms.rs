use permissions::Permission;
use piccolo::{FromValue, Value};
use strict_partial_ord_derive as strict;

#[derive(Debug, Default, PartialEq, Eq, strict::PartialOrd)]
pub struct ReadWritePermissions {
    pub read: bool,
    pub write: bool,
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
