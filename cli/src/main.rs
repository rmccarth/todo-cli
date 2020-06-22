extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::process;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TODO {
    title: String,
    subject: String,
    content: String,
    duration: String,
    status: String
}

fn main() {

    let mut command = String::new();
    let mut title = String::new();
    let mut content = String::new();
    let mut status = String::new();
    let mut subject = String::new();
    let mut duration = String::new();

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Description: A command line utility to track your TODO's!");
        ap.refer(&mut command)
            .add_argument("display", Store, "display all TODO items")
            .add_argument("create", Store, "create a TODO item")
            .add_argument("update", Store, "update a TODO item")
            .add_argument("delete", Store, "delete a TODO item (by title)");
        ap.refer(&mut title)
            .add_argument("title", Store,
                "title of the TODO");
        ap.refer(&mut duration)
            .add_option(&["-d", "--duration"], Store,
                "Estimated Duration of Task");
        ap.refer(&mut content)
            .add_option(&["-d", "--content"], Store,
                "Task Content/Description");
        ap.refer(&mut status)
            .add_option(&["-d", "--status"], Store,
                "Current Status of the Task");
        ap.refer(&mut subject)
            .add_option(&["-s", "--subject"], Store,
                "Category of TODO (e.g. 'work'");
        ap.parse_args_or_exit();
    }
    println!("command: {}", command);
    println!("title: {}", title);

    if command == "create" {
        if content.is_empty() {
            let mut content = "";
        }
        if duration.is_empty() {
            let mut duration = "";
        }
        if subject.is_empty() {
            let mut subject = "";
        }
        if status.is_empty() {
            let mut status = "";
        }
        let todo = TODO { title, subject, content, duration, status};
        println!("TITLE: {}\n SUBJECT: {}\n CONTENT: {}\n DURATION: {}\n STATUS: {}", todo.title, 
            todo.subject, todo.content, todo.duration, todo.status);
        println!("json version:\n");

        let serialized_user = serde_json::to_string(&todo).unwrap();
        println!("{}", serialized_user);
    }
}





