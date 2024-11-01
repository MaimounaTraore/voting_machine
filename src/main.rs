use rusqlite::{Connection, params};  // Import `params` macro here
use std::env;
mod db;
use db::*;  // Import all functions and structs from db.rs
use std::io::{self, Write};


// Here I will define the structs for the different people we have
struct Voter {
    name: String,
    date_of_birth: String,
}

struct Candidate {
    name: String,
    party: String,
    votes: u32, 
}

struct Office {
    name: String,
    candidates: Vec<Candidate>,
}

struct Ballot {
    offices: Vec<Office>,
    is_open: bool, // this will help in opening or closing a ballot
}

struct Vote {
    voter_name: String,
    selected_candidates: Vec<String>, // Here I am not sure if it should be string or a vector meaning if the voter can vote for many people
}
 
//==============================================================================================================================

// Main function
fn main() {
    // Initialize the database
    let conn = initialize_db().expect("Failed to initialize the database.");

    // All ballots are closed on starting point
    let mut ballot = Ballot {
        offices: Vec::new(),
        is_open: false,
    };

    // Main loop for the voting machine, will keep running until the user chooses to exit
    loop {
        println!("\n\n");
        println!("\n\n");
        println!("╔════════════════════════════════════════════════════════════════════════╗");
        println!("║                                                                        ║");
        println!("║  ██     ██  ███████  ██       ██████    █████   ███    ███  ███████    ║");
        println!("║  ██     ██  ██       ██      ██    ██  ██   ██  ████  ████  ██         ║");
        println!("║  ██  █  ██  █████    ██      ██        ██   ██  ██ ████ ██  █████      ║");
        println!("║  ██ ███ ██  ██       ██      ██    ██  ██   ██  ██  ██  ██  ██         ║");
        println!("║   ███ ███   ███████  ███████  ██████    █████   ██      ██  ███████    ║");
        println!("║                                                                        ║");
        println!("║                        TO THE VOTING MACHINE                           ║");
        println!("║                                                                        ║");
        println!("╚════════════════════════════════════════════════════════════════════════╝");
        

        println!("\n\tAre you an admin, or a voter? (Type 'exit' to leave) ");


        // Get the user role input
        let mut user_role = String::new();
        std::io::stdin().read_line(&mut user_role).unwrap();
        let user_role = user_role.trim();

        match user_role {
            "admin" => {
                // Admin login and menu
                if admin_login() {
                    admin_menu(&conn, &mut ballot);
                } else {
                    println!("\tERROR - Authentication failed. Returning to the main menu...");
                }
            },
            "voter" => {
                // Verify voter and proceed to cast vote if election is open
                if verify_voter(&conn) {
                    if ballot.is_open {
                        let candidate_name = get_input("Enter the candidate name you want to vote for:");
                        cast_vote(&conn, &candidate_name);
                    } else {
                        println!("\tSorry, the election is currently closed.");
                    }
                }
            },
            "exit" => {
                // Exit option - break out of the main loop to end the program
                println!("\nExiting the voting machine. Goodbye!");
                break;
            },
            _ => println!("\tSorry, option not recognized. Please try again."),
        }
    }
}

//===========================================================================================================================

// Admin log in function
fn admin_login() -> bool {
    let admin_username = "adminname";
    let admin_password = "adminpassword";

    println!("\n\tEnter admin username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("\n\tEnter admin password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    // Checking if the credentials work
    if username == admin_username && password == admin_password {
        println!("Access granted!");
        true
    } else {
        println!("Error! Access denied!");
        false
    }
}

//===================================================================================================================================

// Admin menu function
// Admin menu function
fn admin_menu(conn: &Connection, ballot: &mut Ballot) {
    loop {
        println!("\t---------Admin Menu------------");
        println!("\t1. Create an election");
        println!("\t2. Register a voter");
        println!("\t3. Open election");
        println!("\t4. Close election");
        println!("\t5. Tally votes");
        println!("\t6. Delete a voter");
        println!("\t7. Delete a candidate");
        println!("\t8. Delete an office");
        println!("\t9. Exit");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                *ballot = create_election(conn);
                println!("\n\t\tGREAT! Election created successfully!");
            },
            "2" => {
                register_voter(conn);
            },
            "3" => open_ballot(ballot),
            "4" => close_ballot(ballot),
            "5" => tally_vote(conn),
            "6" => delete_voter(conn),
            "7" => delete_candidate(conn),
            "8" => delete_office(conn),
            "9" => {
                println!("Exiting admin menu...");
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
    }
}


//=============================================================================================================

// Registering a voter in the database
fn register_voter(conn: &Connection) {
    let name = get_input("\n\tEnter voter's complete name:");
    let date_of_birth = get_input("\n\tEnter voter's date of birth (YYYY-MM-DD): ");

    match db::register_voter(conn, &name, &date_of_birth) {
        Ok(_) => println!("\n\tVoter registered successfully!"),
        Err(err) => println!("Failed to register voter: {}", err),
    }
}

//=============================================================================================================

// Checking if the voter is registered in the database
fn verify_voter(conn: &Connection) -> bool {
    let voter_name = get_input("Enter your name:");
    let voter_dob = get_input("Enter your date of birth (YYYY-MM-DD):");

    match db::is_voter_registered(conn, &voter_name, &voter_dob) {
        Ok(true) => true,
        Ok(false) => {
            println!("\tYou are not registered for voting, SORRY");
            false
        }
        Err(err) => {
            println!("Failed to verify voter: {}", err);
            false
        }
    }
}

//================================================================================================================

// Creating a ballot and storing offices/candidates in the database
fn create_election(conn: &Connection) -> Ballot {
    let mut offices = Vec::new();

    loop {
        let office_name = get_input("\n\tPlease enter the name of the office (President, Judge, or Mayor): ");
        
        // Insert office into database
        conn.execute(
            "INSERT INTO offices (name) VALUES (?1)",
            params![office_name],
        ).expect("Failed to create office");

        let mut candidates = Vec::new();

        // Adding candidates to the specific office that was created
        loop {
            let candidate_name = get_input("\n\tPlease enter the name of the candidate: ");
            let party = get_input("\n\tPlease enter the political party of the candidate: ");

            conn.execute(
                "INSERT INTO candidates (name, party, office_id) VALUES (?1, ?2, (SELECT id FROM offices WHERE name = ?3))",
                params![candidate_name, party, office_name],
            ).expect("Failed to create candidate");

            candidates.push(Candidate {
                name: candidate_name.to_string(),
                party: party.to_string(),
                votes: 0,
            });

            if get_input("\n\tAdd another candidate to the office (type 'yes' or 'no')?:  ").to_lowercase() != "yes" {
                break;
            }
        }
        offices.push(Office {
            name: office_name.to_string(),
            candidates,
        });
        if get_input("\n\tAdd another office to the ballot (type 'yes' or 'no')?:  ").to_lowercase() != "yes" {
            break;
        }
    }
    Ballot { offices, is_open: true }
}

//===================================================================================================================

// Cast a vote for a candidate in the database
fn cast_vote(conn: &Connection, candidate_name: &str) {
    match db::cast_vote(conn, candidate_name) {
        Ok(_) => println!("Vote successfully cast for {}", candidate_name),
        Err(err) => println!("Failed to cast vote: {}", err),
    }
}

//==================================================================================================================================

// Function to retrieve input from the user
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

//===========================================================================================================================

// Function to open the ballot for voting
fn open_ballot(ballot: &mut Ballot) {
    if ballot.is_open {
        println!("Election is already open");
    } else {
        ballot.is_open = true;
        println!("The election has been opened for voting.");
    }
}

// Function to close the ballot for voting
fn close_ballot(ballot: &mut Ballot) {
    if ballot.is_open {
        ballot.is_open = false;
        println!("The ballot has been closed. No voting possible!");
    } else {
        println!("The election is already closed.");
    }
}

//===========================================================================================================================

// Tally votes function using the database
fn tally_vote(conn: &Connection) {
    println!("---------------- ELECTION RESULTS --------------\n");
    let mut stmt = conn.prepare(
        "SELECT o.name AS office_name, c.name AS candidate_name, c.party, c.votes 
         FROM offices o 
         JOIN candidates c ON c.office_id = o.id 
         ORDER BY o.name, c.votes DESC",
    ).expect("Failed to prepare tally query");

    let results = stmt.query_map([], |row| {
        let office_name: String = row.get(0)?;
        let candidate_name: String = row.get(1)?;
        let party: String = row.get(2)?;
        let votes: i32 = row.get(3)?;
        Ok((office_name, candidate_name, party, votes))
    }).expect("Failed to map query results");

    for result in results {
        let (office_name, candidate_name, party, votes) = result.expect("Failed to read row");
        println!("Office: {}\n  {} ({}) - {} votes", office_name, candidate_name, party, votes);
    }
}

//========================================================= DELETION FUNCTIONS ===================================================================


//Deleting a voter
fn delete_voter(conn: &Connection) {
    let voter_name = get_input("\n\tEnter the name of the voter to delete:");

    match conn.execute(
        "DELETE FROM voters WHERE name = ?1",
        params![voter_name],
    ) {
        Ok(deleted) => {
            if deleted > 0 {
                println!("\n\tVoter '{}' deleted successfully!", voter_name);
            } else {
                println!("\n\tNo voter found with that name.");
            }
        }
        Err(err) => println!("Failed to delete voter: {}", err),
    }
}
//=================================================================================================================================

// Deleting a candidate

fn delete_candidate(conn: &Connection) {
    let candidate_name = get_input("\n\tEnter the name of the candidate to delete:");
    let office_name = get_input("\n\tEnter the office the candidate is running for:");

    match conn.execute(
        "DELETE FROM candidates WHERE name = ?1 AND office_id = (SELECT id FROM offices WHERE name = ?2)",
        params![candidate_name, office_name],
    ) {
        Ok(deleted) => {
            if deleted > 0 {
                println!("\n\tCandidate '{}' from office '{}' deleted successfully!", candidate_name, office_name);
            } else {
                println!("\n\tNo candidate found with that name and office.");
            }
        }
        Err(err) => println!("Failed to delete candidate: {}", err),
    }
}
//=================================================================================================================================

// Deleting and office

fn delete_office(conn: &Connection) {
    let office_name = get_input("\n\tEnter the name of the office to delete:");

    // First, delete all candidates associated with this office
    match conn.execute(
        "DELETE FROM candidates WHERE office_id = (SELECT id FROM offices WHERE name = ?1)",
        params![office_name],
    ) {
        Ok(deleted) => {
            println!("\n\tDeleted {} candidates associated with office '{}'.", deleted, office_name);
        }
        Err(err) => println!("Failed to delete candidates: {}", err),
    }

    // Then, delete the office itself
    match conn.execute(
        "DELETE FROM offices WHERE name = ?1",
        params![office_name],
    ) {
        Ok(deleted) => {
            if deleted > 0 {
                println!("\n\tOffice '{}' deleted successfully!", office_name);
            } else {
                println!("\n\tNo office found with that name.");
            }
        }
        Err(err) => println!("Failed to delete office: {}", err),
    }
}

//============================================================================================================================================

