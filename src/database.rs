pub use rusqlite::{params, Connection, Result};


pub fn establish_connection() -> Result<Connection> {
    let conn = Connection::open("habits.db")?;
    // create a table of habits I want to track
    conn.execute(
        "CREATE TABLE IF NOT EXISTS habits (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            importance  INTEGER NOT NULL,
            frequency   TEXT NOT NULL
        )",
        [],
    )?;
    // create a table of daily entries for each habit
    conn.execute(
        "CREATE TABLE IF NOT EXISTS habit_entries (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            habit_id    INTEGER NOT NULL,
            success     BOOLEAN NOT NULL,
            date        TEXT NOT NULL,
            FOREIGN KEY (habit_id) REFERENCES habits(id)
        )",
        [],
    )?;

    Ok(conn)
}

pub struct HabitEntry {
    id: i32,
    habit_id: i32,
    pub success: bool,
    pub date: String,
}


pub struct Habit {
    pub id: i32,
    pub name: String,
    pub importance: i32,
    pub frequency: String,
    pub habit_entries: Vec<HabitEntry>,
}

pub fn add_habit(conn: &Connection, name: &str, importance: i32, frequency: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO habits (name, importance, frequency) VALUES (?1, ?2, ?3)",
        params![name, importance, frequency],
    )?;
    Ok(())
}

pub fn delete_habit(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM habits WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_habit(conn: &Connection, id: i32) -> Result<Habit> {
    let mut stmt = conn.prepare("SELECT id, name, importance, frequency FROM habits WHERE id = ?1")?;
    let habit = stmt.query_map([id], |row| {
        Ok(Habit {
            id: row.get(0)?,
            name: row.get(1)?,
            importance: row.get(2)?,
            frequency: row.get(3)?,
            habit_entries: vec![]
        })
    })?.next().unwrap().unwrap();
    
    let mut stmt = conn.prepare("SELECT id, habit_id, success, date FROM habit_entries WHERE habit_id = ?1")?;
    let habit_entry_iter = stmt.query_map([habit.id], |row| {
        Ok(HabitEntry {
            id: row.get(0)?,
            habit_id: row.get(1)?,
            success: row.get(2)?,
            date: row.get(3)?
        })
    })?;
    
    let mut habit_entries = Vec::new();
    for habit_entry in habit_entry_iter {
        habit_entries.push(habit_entry.unwrap());
    }

    Ok(Habit {
        id: habit.id,
        name: habit.name,
        importance: habit.importance,
        frequency: habit.frequency,
        habit_entries: habit_entries
    })
}

pub fn add_habit_entry(conn: &Connection, habit_id: i32, success: bool, date: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO habit_entries (habit_id, success, date) VALUES (?1, ?2, ?3)",
        params![habit_id, success, date],
    )?;
    Ok(())
}

// pub fn remove_habit(id: i32) -> Result<()> {
//     let conn = Connection::open("habits.db")?;
//     conn.execute("DELETE FROM habits WHERE id = ?1", params![id])?;
//     Ok(())
// }

pub fn get_habits(conn: &Connection) -> Result<Vec<Habit>> {
    // fetch habits and their corresponding entries from the database
    let mut stmt = conn.prepare("SELECT id, name, importance, frequency FROM habits")?;
    let habit_iter = stmt.query_map([], |row| {
        Ok(Habit {
            id: row.get(0)?,
            name: row.get(1)?,
            importance: row.get(2)?,
            frequency: row.get(3)?,
            habit_entries: vec![]
        })
    })?;
    
    let mut habits = Vec::new();

    for habit in habit_iter {
        let habit = habit.unwrap();
        let mut stmt = conn.prepare("SELECT id, habit_id, success, date FROM habit_entries WHERE habit_id = ?1")?;
        let habit_entry_iter = stmt.query_map([habit.id], |row| {
            Ok(HabitEntry {
                id: row.get(0)?,
                habit_id: row.get(1)?,
                success: row.get(2)?,
                date: row.get(3)?
            })
        })?;
        
        let mut habit_entries = Vec::new();
        for habit_entry in habit_entry_iter {
            habit_entries.push(habit_entry.unwrap());
        }

        habits.push(Habit {
            id: habit.id,
            name: habit.name,
            importance: habit.importance,
            frequency: habit.frequency,
            habit_entries: habit_entries
        });
    }
    
    Ok(habits)
}



pub fn get_habit_entries(conn: &Connection, habit_id: i32) -> Result<Vec<HabitEntry>> {
    let mut stmt = conn.prepare("SELECT id, habit_id, success, date FROM habit_entries WHERE habit_id = ?1")?;
    let habit_entry_iter = stmt.query_map([habit_id], |row| {
        Ok(HabitEntry {
            id: row.get(0)?,
            habit_id: row.get(1)?,
            success: row.get(2)?,
            date: row.get(3)?
        })
    })?;
    
    let mut habit_entries = Vec::new();
    for habit_entry in habit_entry_iter {
        habit_entries.push(habit_entry.unwrap());
    }
    
    Ok(habit_entries)
}