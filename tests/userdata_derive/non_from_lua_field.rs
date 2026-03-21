use mlua::{FromLua, UserData};

#[derive(Clone)]
struct NotFromLua(u64);

impl mlua::IntoLua for NotFromLua {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        mlua::IntoLua::into_lua(self.0, lua)
    }
}

#[derive(Clone, FromLua, UserData)]
struct UserDataFieldsNotFromLua {
    #[field_mut]
    f: NotFromLua,
}

fn main() {}
