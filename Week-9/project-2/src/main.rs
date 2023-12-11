use std::io::Write;

fn main() {

    let student_names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matric_numbers = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];
    let levels = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("school.txt").expect("create failed");

    file.write_all(format!("{} , {} , {} , {}\n","Name","Matric Number","Department","Level").as_bytes()).expect("write failed");

    for n in 0..student_names.len() {
        file.write_all(format!("{} , {} , {} , {}\n", student_names[n],matric_numbers[n],departments[n],levels[n]).as_bytes()).expect("write failed");
    }
    
    println!("Student details written in school.txt");    
}    