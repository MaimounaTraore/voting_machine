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
    selected_candidates: vec<String> //Here I am not sure if it should be string or a vector
}

fn main() {
    println!("Hello, world!");
}

fn admin_login() -> bool {
    let admin_username = "admin";
    let admin_password = "password";

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

fn register_voter() -> Voter{

}

fn create_election() -> Ballot{
    let mut offices = Vec::new();

    loop{
        println!("Please enter the name of the office (Presiedent, Judge, or Mayor): ");
        let office_name = String::new();
        std::io::stdin().read_line(&mut office_name).unwrap();
        let office_name = office_name.trim();

        let mut candidates = Vec::new();
        println!("Please enter the name of the candidate: ");

        println!("Please enter the political party of the candidate: ");

    }
}


