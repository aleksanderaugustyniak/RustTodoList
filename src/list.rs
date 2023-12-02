use crate::task::Task;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct List
{
    tasks: Vec<Task>,
}

impl List
{
    pub fn new() -> Self
    {
        List
        {
            tasks: Vec::new()
        }
    }
    
    pub fn add(&mut self, description: String)
    {
        self.tasks.push(Task::new(description));
    }

    pub fn complete(&mut self, index: usize)
    {
        if self.is_valid(index)
        {
            self.tasks[index].complete();
        }
    }
    
    pub fn remove(&mut self, index: usize)
    {
        if self.is_valid(index)
        {
            self.tasks.remove(index);
        }
    }

    fn is_valid(&self, index: usize) -> bool
    {
        let is_index_valid = index < self.tasks.len();
        if !is_index_valid
        {
            eprintln!("Too large index");
        }
        is_index_valid
    }

    pub fn show(&mut self)
    {
        for (i, task) in self.tasks.iter().enumerate()
        {
            print!("{} ", i + 1);
            task.show();
        }
    }

    pub fn to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>>
    {
        let json_content = serde_json::to_string_pretty(self)?;

        std::fs::write(file_path, json_content)?;

        Ok(())
    }

    pub fn from_json(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = std::fs::read_to_string(file_path)?;
        let new_list: List = serde_json::from_str(&json_content)?;
        self.tasks = new_list.tasks;

        Ok(())
    }
}
