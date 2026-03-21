use mlua::{FromLua, Lua};
use mlua_derive::UserData;

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

#[test]
fn test_userdata_derive_error_messages() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/userdata_derive/*.rs");
}
