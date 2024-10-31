use std::env;

//Here I will define the structs for the different people we hav
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

struct Ballot{
    offices: Vec<Office>,
    is_open: bool, //this will help in opening or closisng a ballot
}

struct Vote {
    voter_name: String,
    selected_candidates: Vec<String> //Here I am not sure if it should be string or a vector meaning if the
    //voter can vote for many people
}

//Main function, lol
fn main() {
    
    //All ballots are closed on starting point
    let mut ballot = Ballot {
        offices: Vec::new(),
        is_open: false,
    };
    
    //To store the registered voters so that we can check if a voter is registered before being ablee to vote
    let mut registered_voters: Vec<Voter> = Vec::new();  

    loop{
        println!("\t\t\t------------- WELCOME TO THE VOTING MACHINE --------------\n");
        println!("\n\tAre you an admin or a voter ?(Type 'admin' or 'voter'): ");
        let mut user_role = String::new();
        std::io::stdin().read_line(&mut user_role).unwrap();
        let user_role = user_role.trim();

        match user_role {
            "admin" =>{
                if admin_login() {
                    admin_menu(&mut ballot, &mut registered_voters);
                } else {
                    println!("\tERROR - Authentication failed. Returning to the main menu...");
                }
            },
            "voter" =>{
                if let Some(voter) = verify_voter(&registered_voters) {
                    if ballot.is_open {
                        cast_vote(&voter, &mut ballot);
                    } else {
                        println!("\tSorry, the election is currently closed.");
                    }
                } else {
                    println!("\tYou are not registered for voing, SORRY");
                }
            },
            _ => println!("\tSorry, user not recognized..."),
            }

        }
    }

//Admin log in function
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

    //Checking if the credentials work
    if username == admin_username && password == admin_password{
        println!("Aaccess granted!");
        true
    }else {
        println!("Error! Access denied!");
        false
        }
    }

    //To print the menu to the admin
    fn admin_menu(ballot: &mut Ballot, registered_voters: &mut Vec<Voter>) {
        loop {
            println!("\t---------Admin Menu------------\n\t1. Create an election\n\t2. Register a voter\n\t3. Open election\n\t4. Close election\n\t5. Tally votes\n\t6. Exit");
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();
    
            match choice {
                "1" => {
                    *ballot = create_election();
                    println!("\n\t\tGREAT! Election created successfully!");
                },
                "2" => {
                    register_voter(registered_voters);
                },
                "3" => open_ballot(ballot),
                "4" => close_ballot(ballot),
                "5" => tally_vote(ballot),
                "6" => {
                    println!("Exiting admin menu...");
                    break;
                },
                _ => println!("Invalid choice. Please try again."),
            }
        }
    }
    
    //Checking  if the voter is registered
    fn verify_voter(registered_voters: &Vec<Voter>) -> Option<Voter> {
        let voter_name = get_input("Enter your name:");
        if is_voter_registered(registered_voters, &voter_name) {
            Some(Voter {
                name: voter_name,
                date_of_birth: String::new(), // For simplicity; could be extended
            })
        } else {
            None
        }
    }

//Creating a ballot. I believe I still need to make some changes here
fn create_election() -> Ballot{
    let mut offices = Vec::new();

    loop{
        println!("\n\tPlease enter the name of the office (Presiedent, Judge, or Mayor): ");
        let mut office_name = String::new();
        std::io::stdin().read_line(&mut office_name).unwrap();
        let office_name = office_name.trim();

        let mut candidates = Vec::new();

        //Here, I am adding candidates to the specific office that was created
        loop{
            println!("\n\tPlease enter the name of the candidate: ");
            let mut candidate_name = String::new();
            std::io::stdin().read_line(&mut candidate_name).unwrap();
            let candidate_name = candidate_name.trim();

            println!("\n\tPlease enter the political party of the previous candidate belongs to: ");
            let mut party = String::new();
            std::io::stdin().read_line(&mut party).unwrap();
            let party = party.trim();

            candidates.push(Candidate{
                name: candidate_name.to_string(),
                party: party.to_string(),
                votes : 0,
            });

            println!("\n\tAdd another candidate to the office (type  'yes' or 'no')?:  ");
            let mut ans = String::new();
            std::io::stdin().read_line(&mut ans).unwrap();
            if ans.trim().to_lowercase() != "yes" {
                break;
            }
        }
        offices.push(Office{
            name : office_name.to_string(),
            candidates : candidates,
        });
        println!("\n\tAdd another office to the ballot (type  'yes' or 'no)?:  ");
        let mut ans = String::new();
        std::io::stdin().read_line(&mut ans).unwrap();
        if ans.trim().to_lowercase() != "yes" {
            break;
        }
    }
    Ballot {offices, is_open: true}
}
//Registering a voter
fn register_voter(registered_voters: &mut Vec<Voter>) {
    loop {
        let voter = Voter {
            name: get_input("\n\tEnter voter's complete name:"),
            date_of_birth: get_input("\n\tEnter voter's date of birth (YYYY-MM-DD): "),
        };
        registered_voters.push(voter);
        println!("\n\tVoter registered successfully!");

        let continue_input = get_input("\n\tDo you want to add another voter? (yes/no):");
        if continue_input.trim().to_lowercase() != "yes" {
            break;
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

//This function will check if the voter is registered

fn is_voter_registered(registered_voters: &Vec<Voter>, name: &str) -> bool{
    registered_voters.iter().any(|voter| voter.name == name) //I am not sure, but i guess this can used as a backdoor
}

fn tally_vote(ballot: &Ballot){
    if ballot.is_open {
        println!("Election is still open. Please close the election before tallying votes.");
        return;
    }
    println!("---------------- ELECTION RESULTS --------------\n");
    
    for office in &ballot.offices {
        println!("Office: {}", office.name);
        for candidate in &office.candidates{
            println!("{} ({}) - {}", candidate.name, candidate.party, candidate.votes);
        }
    }
}

//Open the election
fn open_ballot (ballot: &mut Ballot){
    if ballot.is_open {
        println!("Election is already open");
    } else{
        ballot.is_open = true;
        println!("The election has been opened for voting.");
    }
}

//Close the election
fn close_ballot(ballot: &mut Ballot){
    if ballot.is_open{
        ballot.is_open = false;
        println!("The ballot has been closed. No voting possible!");
    }else{
        println!("The election is already close");
    }
}

fn cast_vote(voter: &Voter, ballot: &mut Ballot) {
    if ballot.offices.is_empty() {
        println!("No elections are currently available.");
        return;
    }

    println!("Available elections:");
    for (i, office) in ballot.offices.iter().enumerate() {
        println!("{}. {}", i + 1, office.name);
    }

    let election_choice = get_input("Please select the number of the election you want to vote for:");

    if let Ok(election_index) = election_choice.parse::<usize>() {
        if election_index == 0 || election_index > ballot.offices.len() {
            println!("Invalid election choice.");
            return;
        }

        let selected_office = &mut ballot.offices[election_index - 1];

        println!("Candidates for the {} election:", selected_office.name);
        for (i, candidate) in selected_office.candidates.iter().enumerate() {
            println!("{}. {} ({})", i + 1, candidate.name, candidate.party);
        }

        let candidate_choice = get_input("Please select the number of the candidate you want to vote for:");

        if let Ok(candidate_index) = candidate_choice.parse::<usize>() {
            if candidate_index == 0 || candidate_index > selected_office.candidates.len() {
                println!("Invalid candidate choice.");
                return;
            }

            let selected_candidate = &selected_office.candidates[candidate_index - 1];
            println!(
                "You voted for {} from the {} party in the {} election.",
                selected_candidate.name, selected_candidate.party, selected_office.name
            );
//here we should add like a coounter or somethin to store teh votes for the tallying
        } else {
            println!("Invalid input. Please select a valid candidate number.");
        }
    } else {
        println!("Invalid input. Please select a valid election number.");
    }
}



