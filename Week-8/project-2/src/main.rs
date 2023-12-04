use std::io;

fn main() {
	let mut interviewee : Vec<String> = Vec::new();

	let mut input1 = String::new();
	println!("How many interviewee's are at the interview: ");
	std::io::stdin().read_line(&mut input1).expect("Failed to read line");
	let interviewee_num:i32 = input1.trim().parse().expect("Invalid input");

	let mut max_experience = 0;
	let mut experienced_interviewee = String::new();

	for _count in 0..interviewee_num {
		let mut name = String::new();
		println!("Enter Name: ");
		io::stdin().read_line(&mut name).expect("Failed to read line");

		let mut age = String::new();
		println!("Enter age: ");
		io::stdin().read_line(&mut age).expect("Failed to read line");
		let age:i32 = age.trim().parse().expect("Invalid input");

		let mut experience = String::new();
		println!("How many years of Experience do you have: ");
		io::stdin().read_line(&mut experience).expect("Failed to read line");
		let experience:i32 = experience.trim().parse().expect("Invalid input");
        
        let interviewee_info = format!("Name: {}, Age: {} years, Experience: {} years", name.trim(), age, experience);
        interviewee.push(interviewee_info);

       //check for person with highest experience
       if experience > max_experience{
            max_experience = experience;
            experienced_interviewee = format!("Name: {}, Age: {} years, Experience: {} years", name.trim(), age, experience);
       }	
    }

	// Display the collected information
    println!("Interviewee Information:");
    for info in &interviewee {
        println!("{}", info);
    }

    // Display interviewee with highest experience
    println!("Interviewee with the highest experience is {}", experienced_interviewee);
}