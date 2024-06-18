use std::io;

fn main() {
    println!("Hello welcome to your tod0 - app, what will you like to do today?");

    let mut task_manager = TaskManager { tasks: Vec::new() };

    loop {
        // println!("Enter a task description :");
        println!("press 1 to add a new task, press 2 to view all tasks, press 3 to delete tasks (or 'exit' to quit)");

        // Read user input
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim any extra whitespace/newlines from input
        let input: String = input.trim().to_string();

        if input == "1" {
            task_manager.create_new_task()
        } else if input == "2" {
            task_manager._print_tasks();
        } else if input.to_lowercase() == "exit" {
            break;
        }
    }
}

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
}

// Add a task
// View all tasks
// Mark a task as completed
// Delete a task

impl TaskManager {
    fn _print_tasks(&self) {
        println!("Tasks:");
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }

    fn create_new_task(&mut self) {
        println!("What would you like to do?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let description: String = input.trim().to_string();

        let new_task: Task = Task {
            id: self.tasks.len() as u32 + 1,
            description,
            completed: false,
        };
        self.tasks.push(new_task);
        self._print_tasks()
    }
}
