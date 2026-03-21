use mlua::{FromLua, UserData};

struct NotClone(u64);

impl mlua::IntoLua for NotClone {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        mlua::IntoLua::into_lua(self.0, lua)
    }
}

impl Clone for UserDataFieldsNotClone {
    fn clone(&self) -> Self {
        Self {
            f: NotClone(self.f.0),
        }
    }
}

#[derive(FromLua, UserData)]
struct UserDataFieldsNotClone {
    #[field]
    f: NotClone,
}

fn main() {}
