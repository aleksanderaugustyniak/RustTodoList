use crate::list::List;

pub enum CliState 
{
    Adding,
    Removing,
    Marking,
}

pub struct CliManager
{
    state: CliState,
    list: List,
}

impl CliManager
{
    pub fn new() -> Self
    {
        CliManager
        {
            state: CliState::Adding,
            list: List::new(),
        }
    }

    pub fn run(&mut self)
    {
        if let Err(err) = self.list.from_json("list.json") {
            eprintln!("Error updating from JSON file: {}", err);
        }
        self.list.show();
        println!("Adding, change mode by :a - add, :r - remove, :m - mark done, :e - exit:");

        loop
        {
            let mut input =String::new();
            std::io::stdin().read_line(&mut input).expect("Reading fail");
            if input.trim() == ":e"
            {
                self.handle_exit();
                break;
            }
            if self.change_state(&input)
            {
                continue;
            }
            self.handle_state(&input);
            self.list.show();
        }
    }
 
    fn handle_state(&mut self, input: &String)
    {
        match self.state 
        {
            CliState::Adding => 
            {
                self.handle_adding(&input);
            }
            CliState::Removing => 
            {
                self.handle_removing(&input);
            }
            CliState::Marking => 
            {
                self.handle_marking(&input);
            }
        }
    }

    fn handle_exit(&self)
    {
        if let Err(err) = self.list.to_json("list.json") {
            eprintln!("Error writing to JSON file: {}", err);
        }
    }

    fn handle_adding(&mut self, input: &String)
    {
        self.list.add(input.to_string());
    }
    
    fn handle_action_with_index(&mut self, input: &String, action: impl Fn(&mut CliManager, usize) -> ())
    {
        if let Ok(number) = input.trim().parse::<usize>()
        {
            if number <= 0
            {
                eprintln!("Please provide positive number.");
                return;
            }
            action(self, number - 1);
            return;
        } 
        eprintln!("Invalid input. Please enter a valid number.");
    }
    
    fn handle_marking(&mut self, input: &String)
    {
        self.handle_action_with_index(&input, |manager, index|{
            manager.list.complete(index);
        });
    }
    
    fn handle_removing(&mut self, input: &String)
    {
        self.handle_action_with_index(&input, |manager, index|{
            manager.list.remove(index);
        });
    }
    
    fn change_state(&mut self, input: &String) -> bool
    {
        let trimmed_input = input.trim();
        if trimmed_input == ":a"
        {
            self.state = CliState::Adding;
            println!("Adding, please enter your items: ");
            return true;
        }
        if trimmed_input == ":r"
        {
            self.state = CliState::Removing;
            println!("Removing, please enter your items number: ");
            return true;
        }
        if trimmed_input == ":m"
        {
            self.state = CliState::Marking;
            println!("Marking, please enter your items number: ");
            return true;
        }
        return false;
    }
}
