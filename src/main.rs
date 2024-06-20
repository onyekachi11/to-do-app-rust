use std::io;

fn main() {
    println!("Hello welcome to your tod0 - app, what will you like to do today?");

    let mut task_manager = TaskManager { tasks: Vec::new() };

    loop {
        // println!("Enter a task description :");
        // println!("press 1 to add a new task, press 2 to view all tasks, press 3 to delete tasks, (or 'exit' to quit)");
        println!("Press '1' to add a new task, press '2' to view all tasks,");
        println!("Press '3' to delete tasks, press '4' to mark a tasks");
        println!("(or 'exit' to quit)");

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
        } else if input == "3" {
            task_manager.delete_task()
        } else if input == "4" {
            task_manager.mark_task_as_completed()
        } else if input.to_lowercase() == "exit" {
            break;
        }
    }
}

#[derive(Debug)]
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
        for (index, task) in self.tasks.iter().enumerate() {
            println!(
                "{}). Description: {}, Completed: {}",
                index + 1, // Add 1 to index to start numbering from 1
                task.description,
                task.completed
            );
        }
    }

    fn create_new_task(&mut self) {
        println!("What would you like to do?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: String = input.trim().to_string().clone();

        let new_task: Task = Task {
            id: self.tasks.len() as u32 + 1,
            description: input.clone(),
            completed: false,
        };
        self.tasks.push(new_task);
        println!("Task '{:?}' added succesfully", input.clone())
        // self._print_tasks()
    }

    fn delete_task(&mut self) {
        loop {
            println!(
                "Which item would you like to delete? Type 'cancel' to cancel the delete operation"
            );
            self._print_tasks();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            if input.to_lowercase() == "cancel" {
                break;
            }

            match input.parse::<u32>() {
                Ok(num) => {
                    if self.tasks.iter().any(|task| task.id == num) {
                        self.tasks.retain(|task| task.id != num);
                        println!("Task deleted successfully.");
                        break;
                    } else {
                        println!("Task with ID {} does not exist. Please try again.", num);
                    }
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        }
    }

    fn mark_task_as_completed(&mut self) {
        println!(
            "Which item would you like to mark as completed? Or type 'all' to complete all or 'cancel' to cancel the delete operation"
        );
        self._print_tasks();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: Result<u32, std::num::ParseIntError> = input.trim().parse::<u32>();

        // println!("{input}");

        match input {
            Ok(num) => {
                if num - 1 < self.tasks.len() as u32 {
                    self.tasks[num as usize - 1].completed = true;
                    println!("Task {} marked as completed.", num);
                } else {
                    println!("Invalid task number.");
                }
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }

    // fn mark_all_tasks_as_completed(&mut self) {
    //     for task in &mut self.tasks {
    //         task.completed = true;
    //     }
    // }
}
