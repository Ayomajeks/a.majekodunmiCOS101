use std::fs::File;
use std::io::Write;

fn main() {

    let student_names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matric_numbers = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];
    let levels = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("stout.txt").expect("create failed");

    file.write_all(format!("----------\n"))
}    