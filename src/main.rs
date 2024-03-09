use mlua::prelude::*;
use mlua::Error::FromLuaConversionError;

#[derive(Debug)]
struct FileResource {
    name: String,
    content: String,
}

impl<'lua> FromLua<'lua> for FileResource{
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(t) => {
                let file_name = t.get("name").unwrap();
                let file_content = t.get("content").unwrap();
                let new_file = FileResource {
                    name: file_name,
                    content: file_content,
                };

                // debug so you can see it worked
                println!("File Name: {}", new_file.name);
                println!("File Content: {}", new_file.content);

                return Ok(
                  new_file  
                );
            }
            _ => {Err(FromLuaConversionError { from: ("lua table file"), to: ("FileResource"), message: (Some("shit".into())) })}
        }
    }
    
}

fn create_file(_: &Lua, _: FileResource) -> LuaResult<String> {
    Ok("stuff".into())
}

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    // represent the funtion in lua
    let lua_file_func = lua.create_function(create_file)?;
    
    // pass it into lua environment
    lua.globals().set("file", lua_file_func)?;

    // test creating a file resource from lua
    lua.load(
        r#"
            file{
                name = "some.txt",
                content= "hello world!",
            }
        "#
    ).exec()?;


    Ok(())
}