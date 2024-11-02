use std::io;

mod database;

fn main() {
    println!("Hello sqlite!");
    let conn = database::establish_connection().expect("SQLITE had a problem");
    println!("Database started!");
    // add a habit
    // let _ = database::add_habit(
    //     &conn,
    //     "Exercise",
    //     10,
    //     "Daily",
    // );
    // get all habits
    let habits = database::get_habits(&conn);
    // display the habits
    for habit in habits.expect("There was a problem getting habits") {
        println!("Habit: {}", habit.name);
        println!("Importance: {}", habit.importance);
        println!("Frequency: {}", habit.frequency);
        println!("Habit Entries:");
        for entry in habit.habit_entries {
            println!("Date: {}", entry.date);
            println!("Success: {}", entry.success);
        }
    }
    let habit_selected = false;
    // loop for menu
    loop {
        let mut selection = String::new();
        println!("\n\n\n\n\n");
        if !habit_selected {
            println!("a: add a new habit");
            println!("s: select an existing habit");
            println!("c: mark a habit as complete for today");
            println!("q: quit");
            println!("Your choice:");
            io::stdin().read_line(&mut selection).expect("failed to read line");
            selection = selection.trim().to_string();
            if selection == "a" {
                add_habit(&conn);
            } else if selection == "s" {
                println!("TODO: make a way to select")
            } else if selection == "c" {
                println!("TODO:  mark a habit as complete for today")
            } else if selection == "q" {
                println!("exiting...");
                break;
            } else {
                println!("INVALID SELECTION: please choose a valid option!")
            }
        } else {
            println!("d: delete selected habit");
            println!("c: mark this habit as complete for today");
            println!("x: deselect habit");
            println!("q: quit");
            io::stdin().read_line(&mut selection).expect("failed to read line");
            selection = selection.trim().to_string();
            if selection == "d" {
                println!("TODO: add a way to deselect");
            } else if selection == "c" {
                println!("TODO: add a way to complete selected item");
            } else if selection == "d" {
                println!("TODO: add a way to delete");
            } else if selection == "q" {
                println!("exiting...");
                break;
            } else {
                println!("INVALID SELECTION: please choose a valid option!");
            }
        }
        println!("press enter to continue...");
        io::stdin().read_line(&mut selection).expect("failed to read line");
    }

}

fn add_habit(conn: &database::Connection){
    println!("---ADDING A HABIT---");

    println!("Habit name:");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("failed to read line");
    name = name.trim().to_string();

    println!("Habit frequency:");
    let mut frequency: String = String::new();
    frequency = frequency.trim().to_string();
    io::stdin().read_line(&mut frequency).expect("failed to read line");

    println!("Habit importance:");
    let mut importance: String = String::new();
    io::stdin().read_line(&mut importance).expect("failed to read line");
    importance = importance.trim().to_string();
    let int_importance: i32 = importance.parse().unwrap();

    let _ = database::add_habit(
        &conn,
        &name,
        int_importance,
        &frequency,
    );
    
    println!("Habit added!")
}