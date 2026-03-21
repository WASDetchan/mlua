#[derive(Clone, mlua::FromLua, mlua::UserData)]
struct MyStruct {
    #[field = "name"]
    number: u64,
}

fn main() {}
