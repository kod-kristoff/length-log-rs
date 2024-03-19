xflags::xflags! {
    src "./src/repl/flags.rs"

    cmd repl {
        cmd add-person {
            required name: String
        }
        cmd quit {}
    }
}
// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Repl {
    pub subcommand: ReplCmd,
}

#[derive(Debug)]
pub enum ReplCmd {
    AddPerson(AddPerson),
    Quit(Quit),
}

#[derive(Debug)]
pub struct AddPerson {
    pub name: String,
}

#[derive(Debug)]
pub struct Quit;

impl Repl {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end