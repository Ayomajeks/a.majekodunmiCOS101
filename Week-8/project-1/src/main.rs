use std::io;

fn main() {
    let mut staff: Vec<String> = Vec::new();

    let mut input1 = String::new();
    println!("How many staff are there: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let staff_num: i32 = input1.trim().parse().expect("Invalid input");

    for _count in 0..staff_num {
        let mut name = String::new();
        println!("Enter your Name: ");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        let mut experience_input = String::new();
        println!("How many years of experience do you have: ");
        io::stdin().read_line(&mut experience_input).expect("Failed to read line");
        let experience: i32 = experience_input.trim().parse().expect("Invalid input");

        let mut field = String::new();
        println!("What field are you in: \nlawyer \nacademic \noffice administrator \nteacher ");
        io::stdin().read_line(&mut field).expect("Failed to read line");
        let field = field.trim();

        let staff_details = format!("Name: {}, Experience: {} years, Field: {}", name.trim(), experience, field);
        staff.push(staff_details);

        // Check for the person with the highest experience
        if field == "office administrator" {
            let level = match experience {
                1..=3 => "intern",
                4..=6 => "administrator",
                7..=8 => "senior administrator",
                9..=11 => "office manager",
                12..=13 => "director",
                _ => "CEO",
            };
            println!("Your APS level is {}", level);
        } else if field == "academic" {
            let level = match experience {
                1..=3 => "rookie",
                4..=6 => "Research assistant",
                7..=8 => "PhD candidate",
                9..=11 => "post-doc researcher",
                12..=13 => "Senior lecturer",
                _ => "Dean",
            };
            println!("Your APS level is {}", level);
        } else if field == "teacher" {
            let level = match experience {
                1..=3 => "placement",
                4..=6 => "Classroom teacher",
                7..=8 => "Senior teacher",
                9..=11 => "leading teacher",
                12..=13 => "Deputy Principal",
                _ => "Principal",
            };
            println!("Your APS level is {}", level);
        } else if field == "lawyer" {
            let level = match experience {
                1..=3 => "Paralegal",
                4..=6 => "Junior associate",
                7..=8 => "Associate",
                9..=11 => "senior associate 1-2",
                12..=13 => "senior associate 3-4",
                _ => "partner",
            };
            println!("Your APS level is {}", level);
        } else {
            println!("Not a field of expertise here");
        }
    }
}	                    