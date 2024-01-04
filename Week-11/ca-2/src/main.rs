use std::fs::File;
use std::io::{self, Write};

fn main() {
	let mut username = String::new();
	println!("Let username be first 4 letters of company name ");
	println!("Enter Username: ");
	io::stdin().read_line(&mut username).expect("Failed to read");
	if username.len() < 3 || username.len() > 8 {
		println!("Username cannot be less then 3 nor greater than 8");
	} else {
		println!("Valid Username");
	}

	let mut password = String::new();
	println!("Enter Password: ");
	io::stdin().read_line(&mut password).expect("Failed to read");
	let pass = password.trim().chars().all(|c| c.is_ascii_lowercase() || c.is_numeric());
}

struct Company {
	company_name:String,
	founding_date:i32,
	assets:f32,
	liabilities:f32,
}

impl Company {
	fn calculations(&self)->(f32,f32,f32) {
		let over_leverage = self.assets - self.liabilities / self.assets;
		let percentage_leverages = over_leverage * 100.0;
		let multiple_of_percentage_leverages = percentage_leverages * over_leverage;
	}
}

fn five_percent_of_percentage_leverages(&self)-> f32{
	if liabilities < 10_000_000.0 {
		percentage_leverages * 5/100
	} else {
		println!("liabilities is not less than 10,000,000");
	}
}

fn main() {
    let industry = vec![
    	Company {
    		company_name:String::from("Cadbury"),
    		founding_date:1965,
    		assets:15_000_000.0,
    		liabilities:5_500_000.0,
    	},

    	Company {
    	   company_name:String::from("Champion"),
    	   founding_date:1974,
    	   assets:25_000_000.0,
    	   liabilities:8_000_000.0,	
    	},

    	Company {
    		company_name:String::from("Dangote Sugar"),
    		founding_date:1970,
    		assets:18_000_000.0,
    		liabilities:10_000_000.0,
    	},

    	Company {
    		company_name:String::from("Flour Mills"),
    		founding_date:1960,
    		assets:32_000_000.0,
    		liabilities:4_000_000.0,
    	},

    	Company {
    		company_name:String::from("Nestle"),
    		founding_date:1961,
    		assets:8_000_000.0,
    		liabilities:1_500_000.0,
    	},

    	Company {
    		company_name:String::from("Unilever"),
    		founding_date:1923,
    		assets:37_000_000.0,
    		liabilities:11_000_000.0,
    	},

    	Company {
    		company_name:String::from("Honeywell"),
    		founding_date:1906,
    		assets:34_000_000.0,
    		liabilities:9_000_000.0,
    	},

    	Company {
    		company_name:String::from("Nigerian Breweries"),
    		founding_date:1946,
    		assets:30_000_000.0,
    		liabilities:12_000_000.0,
    	},
    	];

    	let mut file = std::fs::File::create("Nigerian Economy.txt").expect("create failed");

	    file.write_all(format!("{} , {} , {} , {} , {} , {}\n","Company","Founding Date","Assets","Liabilities","Percentage Leverages","5% of percentage leverages").as_bytes()).expect("write failed");

	    for industry in &industry {
            file.write_all(format!("{} , {} , {} , {} , {} , {}\n",company.company_name,company.founding_date,company.assets,company.liabilities,percentage_leverages,five_percent_of_percentage_leverages).as_bytes()).expect("write failed");
	    }
	    println!("Data has been saved to Nigerian.Economy.txt");

	if company.shares > 20_000_000 {
	    let mut file = std::fs::File::create("Data1.txt").expect("create failed");

	    file.write_all("For Companies with Liabilities Less than 10,000,000\n".as_bytes()).expect("write failed");

	    file.write_all(format!("{} , {} , {}\n" ,"Company","Percentage Leverages","Multiple of Percentage Leverages").as_bytes()).expect("write failed");
	    file.write_all(format!("{} , {} , {}\n",company.company_name,percentage_leverages,multiple_of_percentage_leverages).as_bytes()).expect("write failed");
	}

	println!("Data has been saved to Data1.txt");
}