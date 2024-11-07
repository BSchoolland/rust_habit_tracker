use std::io;
use colored::*;
use chrono::prelude::*;
use chrono::Duration;

mod database;

fn main() {
    println!("Hello sqlite!");
    let conn = database::establish_connection().expect("SQLITE had a problem");
    println!("Database started!");

    let mut habit_selected: i32 = -1;
    let mut is_menu_switched: bool = false;
    // loop for menu
    loop {
        let mut selection = String::new();
        println!("\n\n\n\n\n");
        if habit_selected == -1 {
            println!("{}", "---Main Menu---".yellow().bold().underline());
            println!("a: add a new habit");
            println!("s: select an existing habit");
            println!("c: mark a habit as complete for today");
            println!("l: list habits and their status today");
            println!("q: quit");
            println!("Your choice:");
            io::stdin().read_line(&mut selection).expect("failed to read line");
            selection = selection.trim().to_string();
            if selection == "a" {
                add_habit(&conn);
                // show the list of habits
                list_habits(&conn);
            } else if selection == "s" {
                habit_selected = select_habit(&conn);
                is_menu_switched = true;
            } else if selection == "c" {
                // select a habit and complete it
                habit_selected = select_habit(&conn);
                complete_habit(&conn, habit_selected);
                // deselect the habit
                habit_selected = -1;
                // show the list of habits
                list_habits(&conn);
            } else if selection == "l" {
                list_habits(&conn);
            } else if selection == "q" {
                println!("exiting...");
                break;
            } else {
                println!("INVALID SELECTION: please choose a valid option!")
            }
        } else {
            println!("{}", "---Habit Menu---".yellow().bold().underline());
            println!("d: delete selected habit");
            println!("c: mark this habit as complete for today");
            println!("e: edit selected habit");
            println!("x: deselect habit");
            println!("q: quit");
            io::stdin().read_line(&mut selection).expect("failed to read line");
            selection = selection.trim().to_string();
            if selection == "x" {
                habit_selected = -1;
                is_menu_switched = true;
            } else if selection == "c" {
                complete_habit(&conn, habit_selected);
                // show the list of habits
                list_habits(&conn);
            } else if selection == "d" {
                delete_habit(&conn, habit_selected);
                habit_selected = -1;
            } else if selection == "e" {
                println!("TODO: add a way to edit");
            } else if selection == "q" {
                println!("exiting...");
                break;
            } else {
                println!("INVALID SELECTION: please choose a valid option!");
            }
        }
        if !is_menu_switched {
            println!("press enter to continue...");
            io::stdin().read_line(&mut selection).expect("failed to read line");
        } else {
            is_menu_switched = false;
        }
    }

}

fn add_habit(conn: &database::Connection){
    println!("{}", "---Adding a Habit---".yellow().bold().underline());

    println!("Habit name:");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("failed to read line");
    name = name.trim().to_string();

    println!("Habit frequency:");
    let mut frequency: String = String::new();
    io::stdin().read_line(&mut frequency).expect("failed to read line");
    frequency = frequency.trim().to_string();

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

fn delete_habit(conn: &database::Connection, habit_id: i32) {
    // confirm deletion
    println!("Are you sure you want to delete this habit? (y/n)");
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).expect("failed to read line");
    confirm = confirm.trim().to_string();
    if confirm != "y" {
        println!("Deletion cancelled");
        return;
    }
    println!("{}", "Deleting habit...".red());
    let _ = database::delete_habit(&conn, habit_id);
}

fn select_habit(conn: &database::Connection) -> i32 {
    println!("{}", "---Select a Habit---".bold().yellow().underline());

    let habits: Vec<database::Habit> = database::get_habits(&conn).expect("There was a problem getting habits");
    println!("{:<5}{:<15}{:<15}{:<15}", "Num".blue(),"Name".blue(),"Importance".blue(),"Frequency".blue());
    // display the habits
    for (i, habit) in habits.iter().enumerate() {
        println!("{:<5}{:<15}{:<15}{:<15}", i, habit.name, habit.importance, habit.frequency);
    }
    // get the user selection
    println!("Select a habit by number:");
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("failed to read line");
    let selection: usize = selection.trim().parse().expect("Please enter a valid number");
    let habit = habits.get(selection).expect("Invalid selection");
    println!("Habit selected: {}", habit.name.green());
    return habit.id;
}

fn complete_habit(conn: &database::Connection, habit_id: i32) {
    println!("Completing habit...");
    // get today's date
    let local: DateTime<Local> = Local::now();
    let date = local.format("%Y-%m-%d").to_string();
    // add an entry to the habit for today
    let _ = database::add_habit_entry(
        &conn,
        habit_id,
        true,
        &date,
    );
}

fn list_habits(conn: &database::Connection) {
    println!("{}", "---Habit Status---".yellow().bold().underline());
    let habits: Vec<database::Habit> = database::get_habits(&conn).expect("There was a problem getting habits");
    println!("{:<5}{:<15}{:<15}{:<15}{:<15}", "Id".blue(),"Name".blue(),"Importance".blue(),"Frequency".blue(),"Status".blue());
    // display the habits
    for habit in habits {
        let status = if is_habit_complete(&conn, habit.id) {
            "Complete".green()
        } else {
            "Incomplete".red()
        };
        println!("{:<5}{:<15}{:<15}{:<15}{:<15}{:<15}", habit.id, habit.name, habit.importance, habit.frequency, status, "end");
    }
}

fn is_habit_complete(conn: &database::Connection, habit_id: i32) -> bool {
    let habit: database::Habit = database::get_habit(&conn, habit_id).expect("There was a problem getting habit");
    let frequency: i32 = habit.frequency.parse().unwrap();
    // get today's date
    let local: DateTime<Local> = Local::now();
    let today = local.format("%Y-%m-%d").to_string();

    // calculate the start date based on the frequency
    let start_date = (local - Duration::days(frequency as i64)).format("%Y-%m-%d").to_string();

    // check if the habit has an entry within the date range
    let entries: Vec<database::HabitEntry> = database::get_habit_entries(&conn, habit_id).expect("There was a problem getting habit entries");
    for entry in entries {
        if entry.date >= start_date && entry.date <= today {
            return true;
        }
    }
    return false;
}