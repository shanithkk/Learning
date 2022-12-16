mod cli;

use cli::*;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};
use structopt::StructOpt;

fn main() {
    let opt = CliApp::from_args();
    let mut file = fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("./example.json")
        .unwrap();
    let file_data = fs::read_to_string("./example.json").unwrap();
    let mut data: Vec<Students> = Vec::new();
    if !file_data.is_empty() {
        data = serde_json::from_str(&file_data).unwrap();
    }
    match opt.command {
        CommandArgs::Add { name, address } => {
            let vec_len = data.len();
            let mut id = 1;
            if vec_len == 0{
                id;
            }else{
                id  = data[vec_len-1].id + 1;
            }
            let student = Students::new(id, name, address);
            data.push(student);
            match write!(file, "{}", serde_json::to_string(&data).unwrap()) {
                Ok(_) => println!("{:?} is added in file", data),
                Err(e) => println!("Error while writting to the file : {}", e),
            };
        }
        CommandArgs::GetbyName { name } => {
            let mut students = Vec::new();
            for stud in data {
                if stud.name == name {
                    students.push(stud);
                }
            }
            if !students.is_empty() {
                println!("{:?}", students);
            } else {
                println!("No such student");
            }
        }
        CommandArgs::GetbyId { id } => {
            let mut student: Option<Students> = None;
            for stud in data {
                if stud.id == id {
                    student = Some(stud);
                }
            }
            match student {
                Some(student) => println!("{:?}", student),
                None => println!("Student not found"),
            }
        }
        CommandArgs::GetAll => println!("{:?}", data),
        CommandArgs::Delete { id } => {
            let mut counter = 0; 
            let mut deleted_student :Option<Students> = None;
            for (i,s) in data.clone().iter().enumerate(){
                if s.id == id {
                    deleted_student = Some(data.remove(i));
                    counter += 1;
                }
            }
            if counter >0{
                match fs::write("./example.json", serde_json::to_string(&data).unwrap().as_bytes()){
                    Ok(_) => println!("Deleted {:?} ", deleted_student.unwrap()),
                    Err(e) => println!("{}",e),
                };
            } else {
                println!("No student with id {} is present", id);
            }
        },
    }
}

