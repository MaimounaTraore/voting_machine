// src/db.rs

use rusqlite::{params, Connection, Result};

// Voter struct for database interaction
pub struct Voter {
    pub id: i32,
    pub name: String,
    pub date_of_birth: String,
    pub has_voted: bool,
}

// Initialize the database and create tables if they don't exist
pub fn initialize_db() -> Result<Connection> {
    // Open a connection to the SQLite database file
    let conn = Connection::open("voting_machine.db")?;

    println!("Database initialized successfully!");

    // Create voters table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS voters (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             date_of_birth TEXT NOT NULL,
             has_voted BOOLEAN NOT NULL DEFAULT 0
         )",
        [],
    )?;

    // Create offices table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS offices (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL
         )",
        [],
    )?;

    // Create candidates table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS candidates (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             party TEXT NOT NULL,
             votes INTEGER NOT NULL DEFAULT 0,
             office_id INTEGER,
             FOREIGN KEY(office_id) REFERENCES offices(id)
         )",
        [],
    )?;

    Ok(conn) // Return the connection after setting up tables
}

// Register a new voter in the database
pub fn register_voter(conn: &Connection, name: &str, date_of_birth: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO voters (name, date_of_birth, has_voted) VALUES (?1, ?2, ?3)",
        params![name, date_of_birth, false],
    )?;
    println!("Voter {} registered successfully!", name);
    Ok(())
}

// Check if a voter is registered
pub fn is_voter_registered(conn: &Connection, name: &str, dob: &str) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM voters WHERE name = ?1 AND date_of_birth = ?2)")?;
    let exists: bool = stmt.query_row(params![name, dob], |row| row.get(0))?;
    Ok(exists)
}

// Cast a vote by incrementing the vote count for a candidate
pub fn cast_vote(conn: &Connection, candidate_name: &str) -> Result<()> {
    conn.execute(
        "UPDATE candidates SET votes = votes + 1 WHERE name = ?1",
        params![candidate_name],
    )?;
    println!("Vote cast for candidate {}", candidate_name);
    Ok(())
}
