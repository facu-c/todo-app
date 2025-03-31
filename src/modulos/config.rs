

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile { 
    pub id_task: u64,   
    pub name_task : String,
    pub desc_task : String,
    pub done : bool,
}