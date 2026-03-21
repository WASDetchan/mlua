#[derive(Clone, mlua::FromLua, mlua::UserData)]
struct MyStruct(#[field] u64);

fn main() {}
