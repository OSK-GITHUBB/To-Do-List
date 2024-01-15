#[derive(Clone, Debug)]

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

static mut TODO_LIST: Vec<Task> = Vec::new();

    // add_task(description: &str) -> Task: 
    // This function should take a task description as an argument, 
fn add_task(description: &str) -> Task {

    // create a new Task instance with a unique ID, 

    let task: Task = Task {
        id: unsafe { TODO_LIST.len() as u32} + 1,
        description: description.to_string(),
        completed: false     // mark it as not completed, 
    
    };

    println!("task {} initiated", task.id);

    // add it to the vector, 
    unsafe { 
        TODO_LIST.push(task.clone()) 
    };

    println!("added to vector");

    // and return the created Task.
    return task;

}

fn complete_task(id: u32) -> Option<Task> {
    unsafe {
        for task in &mut TODO_LIST {
            if task.id == id {
                task.completed = true;
                println!("Task {id} marked as completed");
                return Some(task.clone()); // Clone the task and return it
            }
        }
    }
    None
}

//list_tasks(): This function should print the details of all tasks in the ToDo list, 
//including their ID, description, and completion status.

fn list_tasks() {
    unsafe {
        println!("To-do List: {:?}", TODO_LIST);
    }
}

fn main() {
    add_task("Todo 1: take out the garbage.");

    list_tasks();

    complete_task(1);

    list_tasks();

    add_task("Todo 2: make the bed.");

    list_tasks();

    complete_task(2);

    list_tasks();
}