use std::env;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
#[derive(Serialize, Deserialize)]
struct Task{
    id: usize,
    title: String, //task desc, owned text
    completed: bool,
}
fn load_tasks() -> Vec<Task>{
    let path = "tasks.json";

    if !Path::new(path).exists(){
        return Vec::new();
    }
    let data = fs::read_to_string(path).expect("Failed to read this file"); //read contents of the file and return it as a String
    serde_json::from_str(&data).expect("Failed to parse JSON") //convert the JSON text into Vec<Task> format
}
fn save_tasks(tasks: &Vec<Task>){
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialise the tasks"); //convert the Vec<Task> into formatted JSON
    fs::write("tasks.json",data).expect("Failed to write the file");
}
fn reassign_ids(tasks: &mut Vec<Task>){
    for (index,task) in tasks.iter_mut().enumerate(){
        task.id = index+1;
    }
}
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len()<2{
        println!("Bro, give some command");
        return;
    }
    let command = &args[1];
    let mut tasks = load_tasks();
    

    match command.as_str(){
        "add" => {
            if args.len()<3{
                println!("Usage: todo add /task name");
                return;
            }
            let title = String::from(&args[2]);
            let task = Task{
                id: tasks.len()+1,
                title,
                completed: false,
            };

            tasks.push(task);
            println!("Task added");
        }
        "list" => {
            for task in &tasks{
                println!("{}: {} [{}]", task.id, task.title, task.completed);
            }
        }
        
        "done" => {
        if args.len() < 3{
            println!("What is this bro, add id after that");
            return;
        }
        let id: usize = args[2].parse().expect("Invalid ID");
        let mut found = false;

        for task in &mut tasks{
            if task.id == id{
                task.completed = true;
                found = true;
                break;
            }
        }
        if found{
            save_tasks(&tasks);
            println!("Task {} completed, well done!",id);
        }else{
            println!("Task not found");
        }
    }
    "delete" => {
        if args.len()<3{
            println!("Bro add the delete id");
            return;
        }
        let id: usize = args[2].parse().expect("Invalid ID");
        if let Some(index) = tasks.iter().position(|task| task.id == id){
            tasks.remove(index);
            reassign_ids(&mut tasks);
            save_tasks(&tasks);
            println!("Task {} deleted successfully", id);
        } else {
            println!("Bro the task is not there");
        }
    }
    "edit" => {
        if args.len()<5{
            println!("give more arguments bro");
            return;
        }
        let id: usize = args[2].parse().expect("Invalid ID");
        let new_title = String::from(&args[3]);
        let new_status: bool = args[4].parse().expect("Invalid status");

        let mut found = false;

        for task in &mut tasks{
            if task.id == id{
                task.title = new_title;
                task.completed = new_status;
                found = true;
                break;
            }
            
        }
        if found{
                save_tasks(&tasks);
                println!("Task {} updated successfully!", id);
            }else{
                println!("Task not found");
            }
    }
    _ => println!("What the f is this?!"),
    }
    
    save_tasks(&tasks);
}