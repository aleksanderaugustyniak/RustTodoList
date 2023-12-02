use serde::Serialize;
use serde::Deserialize;
use colored::*;

#[derive(Serialize, Deserialize)]
pub struct Task
{
    description: String,
    completed: bool,
}

impl Task
{
    pub fn new(description: String) -> Self
    {
        Task
        {
            description: description.to_lowercase(), 
            completed: false,
        }
    }

    pub fn show(&self)
    {
        let status = match self.completed {
            true => String::from("done: ").green(),
            false => String::from("todo: ").red(),
        };
        print!("{}{}", status , self.description.to_string());
    }
    
    pub fn complete(&mut self)
    {
        self.completed = !self.completed;
    }
}