use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("Create Epic");
    println!("```");
    println!("----------------------------");
    println!("Epic Name:");
    let name = get_user_input();
    println!("Epic Description:");
    let description = get_user_input();
    println!("```");

    Epic::new(name, description)
}

fn create_story_prompt() -> Story {
    println!("Create Story");
    println!("```");
    println!("----------------------------");
    println!("Story Name:");
    let name = get_user_input();
    println!("Story Description:");
    let description = get_user_input();
    println!("```");

    Story::new(name, description)
}

fn delete_epic_prompt() -> bool {
    println!("Delete Epic");
    println!("```");
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]: ");
    let agree_str = get_user_input();

    agree_str.to_lowercase() == "y"
}

fn delete_story_prompt() -> bool {
    println!("Delete Story");
    println!("```");
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]: ");
    let agree_str = get_user_input();

    agree_str.to_lowercase() == "y"
}

fn update_status_prompt() -> Option<Status> {
    println!("Update Status");
    println!("```");
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    let status_num_str = get_user_input();
    let num = status_num_str.parse::<usize>().unwrap();

    return match num {
        1 => Some(Status::Open),
        2 => Some(Status::InProgress),
        3 => Some(Status::Resolved),
        4 => Some(Status::Closed),
        _ => None
    }
}