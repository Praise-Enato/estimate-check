use std::io;

// One task: its name, the hours I guessed, and the hours it really took.
struct Task {
    name: String,
    estimate: f64,
    actual: f64,
}

fn main() {
    // The list grows as tasks are added, so it needs mut.
    let mut tasks: Vec<Task> = Vec::new();

    println!("Welcome to Estimate Check!");
    println!("See how close your time estimates are to the real time.");

    loop {
        print_menu();
        let choice = read_text("Choose an option (1-3):");

        match choice.as_str() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("'{}' is not an option. Please enter 1, 2 or 3.", choice),
        }
    }
}

// Prints the main menu.
fn print_menu() {
    println!();
    println!("===== Estimate Check =====");
    println!("1. Add a task");
    println!("2. List my tasks");
    println!("3. Quit");
}

// Shows a question and returns what the user typed, without spaces or the Enter key.
fn read_text(question: &str) -> String {
    println!("{}", question);

    let mut input = String::new();
    let bytes = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // 0 bytes means there is no more input (Ctrl+D), so stop the program.
    if bytes == 0 {
        println!("Goodbye!");
        std::process::exit(0);
    }

    input.trim().to_string()
}

// Keeps asking until the user types a name that is not empty.
fn read_name() -> String {
    loop {
        let name = read_text("Task name:");

        if name.is_empty() {
            println!("The name cannot be empty. Try again.");
        } else {
            return name;
        }
    }
}

// Keeps asking until the user types a number of hours above 0 and up to 1000.
fn read_hours(question: &str) -> f64 {
    loop {
        let input = read_text(question);

        let hours: f64 = match input.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("'{}' is not a number. Try again.", input);
                continue;
            }
        };

        if hours > 0.0 && hours <= 1000.0 {
            return hours;
        }
        println!("Hours must be more than 0 and no more than 1000. Try again.");
    }
}

// Asks for a new task and adds it to the list.
// It takes &mut because it changes the list.
fn add_task(tasks: &mut Vec<Task>) {
    let name = read_name();
    let estimate = read_hours("Estimated hours:");
    let actual = read_hours("Actual hours:");

    let task = Task {
        name,
        estimate,
        actual,
    };

    // push takes ownership of task, so now the Vec owns it.
    tasks.push(task);

    println!("Task added. You now have {} task(s).", tasks.len());
}

// Works out how far the actual time was from the estimate, as a percent.
fn percent_off(estimate: f64, actual: f64) -> f64 {
    (actual - estimate) / estimate * 100.0
}

// Turns a percent into words, like "50% over" or "right on time".
fn describe(percent: f64) -> String {
    if percent == 0.0 {
        return String::from("right on time");
    }

    let word = if percent > 0.0 { "over" } else { "under" };
    format!("{:.0}% {}", percent.abs(), word)
}

// Prints every task with how far off its estimate was.
// It only reads the list, so it borrows it with &.
fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks yet. Choose 1 to add one.");
        return;
    }

    println!();
    println!("Your tasks:");

    let mut number = 1;
    for task in tasks {
        let percent = percent_off(task.estimate, task.actual);

        println!(
            "{}. {} - estimated {:.1} h, actual {:.1} h, {}",
            number,
            task.name,
            task.estimate,
            task.actual,
            describe(percent)
        );
        number += 1;
    }

    print_summary(tasks);
}

// Adds up all the tasks and prints the totals with a short tip.
fn print_summary(tasks: &Vec<Task>) {
    let mut total_estimate = 0.0;
    let mut total_actual = 0.0;
    let mut over_count = 0;

    for task in tasks {
        total_estimate += task.estimate;
        total_actual += task.actual;

        if task.actual > task.estimate {
            over_count += 1;
        }
    }

    let overall = percent_off(total_estimate, total_actual);

    println!();
    println!("Total estimated: {:.1} h", total_estimate);
    println!("Total actual:    {:.1} h", total_actual);
    println!("Overall you were {}.", describe(overall));
    println!(
        "You went over your estimate on {} of {} tasks.",
        over_count,
        tasks.len()
    );

    if overall > 20.0 {
        println!("Tip: tasks take you longer than you think. Add extra time to your estimates.");
    } else if overall < -20.0 {
        println!("Tip: you finish faster than you think. You can estimate a little less.");
    } else {
        println!("Nice! Your estimates are close to the real time.");
    }
}
