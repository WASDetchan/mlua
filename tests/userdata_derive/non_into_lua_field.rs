use mlua::{FromLua, UserData};

#[derive(Clone)]
struct NotIntoLua(u64);

#[derive(Clone, FromLua, UserData)]
struct UserDataFieldsNotIntoLua {
    #[field]
    f: NotIntoLua,
}

fn main() {}
