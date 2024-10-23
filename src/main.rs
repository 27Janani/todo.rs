use std::io::{self, Write};

struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
        }
    }
}

struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp { tasks: Vec::new() }
    }

    fn add_new_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn mark_task_as_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
        } else {
            println!("Invalid task index.");
        }
    }

    fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{}: {} {}", index + 1, status, task.description);
        }
    }
}

fn main() {
    let mut todo_list_app = TodoApp::new();

    loop {
        println!("1. Add New Task");
        println!("2. Mark Task As Done");
        println!("3. Show Tasks");
        println!("4. Exit");

        let choice = match get_numeric_input("Enter your choice:") {
            Some(value) => value,
            None => {
                println!("Invalid input, please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => {
                let description = get_string_input("Enter task description: ");
                todo_list_app.add_new_task(&description);
            }
            2 => {
                let index = match get_numeric_input("Enter the task index to mark as done:") {
                    Some(value) => value as usize,
                    None => {
                        println!("Invalid input, please enter a proper number.");
                        continue;
                    }
                };
                todo_list_app.mark_task_as_done(index - 1);
            }
            3 => todo_list_app.show_tasks(),
            4 => break,
            _ => println!("Invalid option, enter a number between 1-4"),
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<u8> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<u8>() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            None
        }
    }
}
