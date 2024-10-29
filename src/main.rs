use std::env;

//Here I will define the structs for the different people we hav
struct Voter {
    name: String,
    date_of_birth: String,
}

struct Candidate {
    name: String,
    party: String,
}

struct Office {
    name: String,
    candidates: Vec<Candidate>,
}

struct Ballot{
    offices: Vec<Offices>,
    is_open: bool, //this will help in opening or closisng a ballot
}

struct vote {
    voter_name: Strig,
    selected_candidates: vec<String> //Here I am not sure if it should be string or a vector meaning if the
    //voter can vote for many people
}

//Main function, lol
fn main() {
    
    //All ballots are closed on starting point
    let mut ballot = Ballot {
        offices: Vec::new(),
        is_open: false,
    };
    
    //Here we start the program with the admin bieng able to log in
    if admin_login (){
        loop{
            println!("Choose 1. Create an election\n2. Register a voter\n3. Open election\n4. Close election\n5. Tally votes\n6. Exit ");
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            //To store the registered voters so that we can check if a voter is registered befrpe being ab;e to vote
            let mut registered_voters: Vec<Voter> = Vec::new();  

            match choice{
                "1" => {
                    let ballot = create_election();
                    println!("Elections created successfully!");
                }
                "2" => {
                    let voter = register_voter();
                    println!("Voter registered successfully!");
                }
                "3" => open_election(&mut ballot);
                "4" => close_election(&mut ballot);
                "5" => tally_votes(&ballot);
                "6" =>{
                    println!("Exiting the program...");
                    break;
                }
                _ => println!("Invalid choice. Please try again!");
            }
        }
    }
}

//Admin log in function
fn admin_login() -> bool {
    let admin_username = "adminname";
    let admin_password = "adminpassword";

    println!("Enter admin username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("Enter admin password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    //Checking if the credentials work
    if username == admin_username && password == admin_password{}
        println!("Aaccess granted!");
        true
    }else {
        println!("Error! Access denied!");
        false
}
//Creating a ballot. I believe I still need to make some changes here
fn create_election() -> Ballot{
    let mut offices = Vec::new();

    loop{
        println!("Please enter the name of the office (Presiedent, Judge, or Mayor): ");
        let office_name = String::new();
        std::io::stdin().read_line(&mut office_name).unwrap();
        let office_name = office_name.trim();

        let mut candidates = Vec::new();

        //Here, I am adding candidates to the specific office that was created
        loop{
            println!("Please enter the name of the candidate: ");
            let mut candidate_name = String::new();
            std::io::stdin().read_line(&mut candidate_name).unwrap();
            let candidate_name = candidate_name.trim();

            println!("Please enter the political party of the candidate: ");
            let party = String::new();
            std::io::stdin().read_line(&mut party).unwrap();
            let party = party.trim();

            candidates.push(Candidate{
                name: candidate_name,
                party: party,
            });

            println!("Add another candidate to the office (type 'Y' for yes and 'N' for no)?:  ");
            let mut ans = String::new();
            std::io::stdin().read_line(&mut ans).unwrap();
            if ans.trim().to_lowercase() != "yes" {
                break;
            }
        }
        offices.push(Office{
            name : office_name,
            candidates : candidates,
        });
        println!("Add another office to the ballot (type 'Y' for yes and 'N' for no)?:  ");
        let mut ans = String::new();
        std::io::stdin().read_line(&mut ans).unwrap();
        if ans.trim().to_lowercase() != "yes" {
            break;
        }
    }
    Ballot {offices}
}
//Registering a voter
fn register_voter(registered_voters: &mut Vec<Voter>){
    let voter = Voter {
        name: get_input("Enter voter's name:"),
        date_of_birth: get_input("Enter voter's date of birth (YYYY-MM-DD):"),
    };
    registered_voters.push(voter);
    println!("Voter registered successfully!");
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
    
    for office in ballot.offices {
        println!("Office: {}", office.name);
        for candidate in office.candidates{
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

fn cast_vote(registered_voters: &Vec<Voter>, ballot: &mut Ballot){
    if !ballot.is_open {
        println!("Sorry, you cannot vote right now as the election is not open for voting!");
        return;
    }
    let voter_name = get_input("Enter your name: ");
    if !is_voter_registered(registered_voters, &voter_name){
        println!("Sorry, you are not elligible to vote as you are not a registered voter!");
        return;

        let mut selected_candidates = Vec::new();

        for office in &mut ballot.offices {
            println!("Office: {}", office.name);
            for (i, candidate) in office.candidates.iter().enumerate() {
                println!("{}. {} ({})", i + 1, candidate.name, candidate.party);
            }
            let choice = get_input("Enter the number of your selected candidate:");
            let choice: usize = choice.trim().parse().unwrap_or(0);
            if choice > 0 && choice <= office.candidates.len() {
                selected_candidates.push(office.candidates[choice - 1].name.clone());
                office.candidates[choice - 1].votes += 1;  // Increment vote for the selected candidate
            } else {
                println!("Invalid choice.");
            }
        }
    
        println!("Thank you, {}! Your vote has been cast.", voter_name);
    //This code needs refinement and matching with main.
    //Main should show at first are you admin or voter than display optiosn according to that
}



