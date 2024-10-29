use std::env;

//Here I will define the structs for the different people we have

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
}

struct vote {
    voter_name: Strig,
    selected_candidates: vec<String> //Here I am not sure if it should be string or a vector meaning if the
    //voter can vote for many people
}

fn main() {
    //Here we start the program with the admin bieng able to log in
    if admin_login (){
        loop{
            println!("Choose 1. Create an election\n2. Register a voter\n3. Exit: ");
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice{
                "1" => {
                    let ballot = create_election();
                    println!("Elections created successfully!");
                }
                "2" => {
                    let voter = register_voter();
                    println!("Voter registered successfully!");
                }
                "3" =>{
                    println!("Exiting the program...");
                    break;
                }
                _ => println!("Invalid choice. Please try again!");
            }
        }
    }
}

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

fn register_voter() -> Voter {
    println!("Enter the name of the voter: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("Enter the date of birth of the voter in the format YYYY-MM-DD: ");
    let mut dob = String::new();
    std::io::stdin().read_line(&mut voter_name).unwrap();
    let voter_name = voter_name.trim();

    Voter{
        name: name,
        date_of_birth : dob,
    }
}




