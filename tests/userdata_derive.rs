use std::sync::Arc;

use mlua::prelude::{LuaUserDataFields, LuaUserDataMethods};
use mlua::userdata::{MetaMethod, UserData};
use mlua::{FromLua, Lua};
use mlua_derive::UserData;

pub trait UserDataImpl: Sized {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F);
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M);
}
//
// #[derive(Clone, UserData)]
// struct MyStruct {
//     #[field]
//     a: u64,
//     #[field_mut]
//     b: u64,
// }
//
// #[mlua::userdata_impl]
// impl MyStruct {
//     #[field_get(name = "s", infallible)]
//     fn sum(&self) -> u64 {
//         self.a + self.b
//     }
//
//     #[method(infallible)]
//     fn double_a(&mut self) {
//         self.a *= 2;
//     }
//
//     #[method]
//     fn parse_and_set_b(&mut self, _: &mlua::Lua, b: String) -> mlua::Result<()> {
//         self.b = b.parse().map_err(|e| mlua::Error::ExternalError(Arc::new(e)))?;
//         Ok(())
//     }
//
//     #[meta_add(infallible)]
//     fn add(&self, _: &mlua::Lua, other: Self) -> Self {
//         Self {
//             a: self.a + other.a,
//             b: self.b + other.b,
//         }
//     }
// }

#[derive(Clone, FromLua)]
struct MyStruct {
    a: u64,
    b: u64,
}

impl MyStruct {
    fn sum(&self) -> u64 {
        self.a + self.b
    }

    fn double_a(&mut self) {
        self.a *= 2;
    }

    fn parse_and_set_b(&mut self, _: &mlua::Lua, b: String) -> mlua::Result<()> {
        self.b = b.parse().map_err(|e| mlua::Error::ExternalError(Arc::new(e)))?;
        Ok(())
    }

    fn add(&self, _: &mlua::Lua, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl UserData for MyStruct
where
    MyStruct: UserDataImpl,
{
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        <MyStruct as UserDataImpl>::add_fields(fields);
        fields.add_field_method_get("a", |_, this| Ok(this.a));
        fields.add_field_method_get("b", |_, this| Ok(this.b));
        fields.add_field_method_set("b", |_, this, b: u64| {
            this.b = b;
            Ok(())
        });
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        <MyStruct as UserDataImpl>::add_methods(methods);
    }
}

impl UserDataImpl for MyStruct {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("s", |_, this| Ok(this.sum()));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("double_a", |_, this, _: ()| {
            this.double_a();
            Ok(())
        });
        methods.add_method_mut("parse_and_set_b", |lua, this, (b,): (String,)| {
            this.parse_and_set_b(lua, b)
        });

        methods.add_meta_method(MetaMethod::Add, |lua, this, (other,): (Self,)| {
            Ok(this.add(lua, other))
        });
    }
}

#[test]
fn test_userdata_derive() {
    let lua = Lua::new();
    lua.globals().set("A", MyStruct { a: 1, b: 0 }).unwrap();
    lua.globals().set("B", MyStruct { a: 2, b: 4 }).unwrap();
    let chunk = lua.load(
        r#"
    A:double_a()
    B:parse_and_set_b "15"
    A.b = A.a
    C = A + B
    return C.s
    "#,
    );
    assert_eq!(chunk.eval::<f64>().unwrap(), 21.0);
}

#[derive(Clone, FromLua, UserData)]
struct UserDataFields {
    #[field]
    f: u64,
    #[field(name = "g")]
    k: u64,
}

#[test]
fn test_userdata_fields() {
    let lua = Lua::new();
    lua.globals().set("A", UserDataFields { f: 1, k: 2 }).unwrap();
    let chunk = lua.load(
        r#"
    return A.f + A.g
    "#,
    );
    assert_eq!(chunk.eval::<f64>().unwrap(), 3.0);
}

#[derive(Clone, FromLua, UserData)]
struct UserDataFieldsMut {
    #[field_mut]
    f: u64,
    #[field_mut(name = "g")]
    k: u64,
}

#[test]
fn test_userdata_fields_mut() {
    let lua = Lua::new();
    lua.globals().set("A", UserDataFieldsMut { f: 1, k: 2 }).unwrap();
    let chunk = lua.load(
        r#"
    A.f = 15
    A.g = 21
    return A.f + A.g
    "#,
    );
    assert_eq!(chunk.eval::<f64>().unwrap(), 36.0);
}
