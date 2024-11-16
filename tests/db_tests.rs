use voting_machine::db::{initialize_db, register_voter, is_voter_registered, cast_vote, record_vote, has_voted};
use rusqlite::{Connection, params};

// Helper function to create an in-memory database for testing
// This function initializes a new database connection
fn create_test_connection() -> Connection {
    let conn = initialize_db().unwrap(); // Corrected to match the function signature
    conn
}

#[test]
fn test_initialize_db() {
    // Test that the database initializes successfully
    let conn = create_test_connection();
    assert!(conn.is_autocommit(), "Database should initialize correctly with autocommit mode.");
}

#[test]
fn test_register_voter() {
    let conn = create_test_connection();
    let name = "John Doe";
    let dob = "01/01/1990";

    register_voter(&conn, name, dob).unwrap();
    let exists = is_voter_registered(&conn, name, dob).unwrap();
    assert!(exists, "Voter should be registered successfully.");
}

#[test]
/*fn test_cast_vote() {
    // Test casting a vote for a candidate
    // Scenario: A candidate exists, and we cast a vote for them. Their vote count should increase by 1.
    let conn = create_test_connection();
    let office_name = "President"; // Name of the office
    let candidate_name = "Alice"; // Candidate's name

    // Insert the office into the database
    conn.execute("INSERT INTO offices (name) VALUES (?1)", params![office_name]).unwrap();
    // Insert the candidate into the database, associating them with the office
    conn.execute(
        "INSERT INTO candidates (name, party, office_id) VALUES (?1, ?2, (SELECT id FROM offices WHERE name = ?3))",
        params![candidate_name, "Party A", office_name],
    )
    .unwrap();

    // Cast a vote for the candidate
    cast_vote(&conn, candidate_name).unwrap();

    // Check the updated vote count for the candidate
    let vote_count: i32 = conn
        .query_row("SELECT votes FROM candidates WHERE name = ?1", params![candidate_name], |row| row.get(0))
        .unwrap();
    assert_eq!(vote_count, 1, "Vote count should increment correctly."); // Assert that the vote count is now 1
}*/

#[test]
fn test_register_duplicate_voter() {
    // Test handling duplicate voter registration
    // Scenario: Attempting to register a voter with the same name and date of birth should not cause errors.
    let conn = create_test_connection();
    let name = "Jane Doe"; // Voter's name
    let dob = "02/02/1992"; // Voter's date of birth

    register_voter(&conn, name, dob).unwrap(); // Register the voter for the first time
    let result = register_voter(&conn, name, dob); // Attempt to register the same voter again
    assert!(result.is_ok(), "Duplicate registration should not cause an error."); // Assert that no error occurs
}

#[test]
fn test_has_voted() {
    // Test checking if a voter has already voted for a specific office
    // Scenario: A voter votes for an office, and subsequent checks should indicate they've already voted.
    let conn = create_test_connection();
    let voter_name = "John Doe"; // Voter's name
    let voter_dob = "01/01/1990"; // Voter's date of birth
    let office_name = "President"; // Office name

    // Register the voter
    register_voter(&conn, voter_name, voter_dob).unwrap();
    // Insert the office into the database
    conn.execute("INSERT INTO offices (name) VALUES (?1)", params![office_name]).unwrap();

    // Retrieve the voter and office IDs for associating votes
    let voter_id = conn
        .query_row("SELECT id FROM voters WHERE name = ?1", params![voter_name], |row| row.get(0))
        .unwrap();
    let office_id = conn
        .query_row("SELECT id FROM offices WHERE name = ?1", params![office_name], |row| row.get(0))
        .unwrap();

    // Record the voter's vote for the office
    record_vote(&conn, voter_id, office_id).unwrap();
    // Check if the voter has already voted for the office
    let already_voted = has_voted(&conn, voter_id, office_id).unwrap();
    assert!(already_voted, "Voter should be marked as having voted."); // Assert that the voter is marked as having voted
}
