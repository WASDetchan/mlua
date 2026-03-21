#[derive(Clone, mlua::FromLua, mlua::UserData)]
struct MyStruct(#[field(name = 15)] u64);

fn main() {}
