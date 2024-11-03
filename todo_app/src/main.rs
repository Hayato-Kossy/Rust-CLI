use std::io;
use std::io::Write;

struct Task {
    id: i32,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() as i32;
        let task = Task {
            id,
            description,
            completed: false,
        };
        self.tasks.push(task);
    }

    fn complete_task(&mut self, id: i32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
            }
        }
    }

    fn print(&self) {
        for task in &self.tasks {
            let status = if task.completed { "X" } else { " " };
            println!("[{}] {}", status, task.description);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }
        else if input == "print" {
            todo_list.print();
        }
        else if input.starts_with("add ") {
            let description = input[4..].to_string();
            todo_list.add_task(description);
        }
        else if input.starts_with("complete ") {
            let id = input[9..].parse().unwrap();
            todo_list.complete_task(id);
        }
        else {
            println!("Invalid command");
        }
    }
}
