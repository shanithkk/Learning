use super::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct CliApp{
    #[structopt(subcommand)]
    pub command : CommandArgs
}

#[derive(StructOpt, Debug)]
pub enum CommandArgs{
    Add { name : String, address: String},
    GetbyName {name: String},
    GetbyId{ id : u32},
    GetAll,
    Delete{id : u32},
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Students {
    pub id: u32,
    pub name: String,
    address: String,
}

impl Students {
    pub fn new(id: u32, name: String, address: String) -> Students {
        Students { id, name, address }
    }
}
