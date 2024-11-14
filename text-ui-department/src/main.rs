use std::collections::HashMap;

fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();

    cmd("Add Sally to Engineering", &mut db);
    cmd("Add Sayid to Engineering", &mut db);
    cmd("Add John to Engineering", &mut db);
    cmd("Add Michael to Ship", &mut db);
    cmd("Add Jack to Management", &mut db);

    cmd("Read Engineering", &mut db);
    cmd("All", &mut db);
}

fn cmd(command: &str, db: &mut HashMap<String, Vec<String>>) {
    let mut iter = command.split_whitespace();
    let op = iter.next();
    match op {
        None => {
            println!("No operator");
        }
        Some(op) => match op {
            "Add" => {
                let name = iter.next();
                match name {
                    None => {
                        println!("Invalid command: no name provided");
                    }
                    Some(name) => {
                        iter.next();
                        let department = iter.next();
                        match department {
                            None => {
                                println!("Invalid command: No department provided");
                            }
                            Some(department) => {
                                let members =
                                    db.entry(String::from(department)).or_insert(Vec::new());
                                members.push(String::from(name));
                            }
                        }
                    }
                }
            }
            "Read" => {
                let mut all: Vec<String> = Vec::new();
                let department = iter.next();
                match department {
                    None => {
                        println!("Invalid command: Provide a department to read")
                    }
                    Some(k) => {
                        let default: Vec<String> = vec![];
                        let v = db.get(k).unwrap_or(&default);
                        for i in v {
                            all.push(i.to_string());
                        }
                        all.sort();
                        println!("{k} members: {}", all.join(", "))
                    }
                }
            }
            "All" => {
                let mut all: Vec<String> = Vec::new();
                for (_, v) in db {
                    all.append(&mut v.clone());
                }
                all.sort();
                println!("Company members: {}", all.join(", "))
            }
            &_ => {
                println!("Invalid operator")
            }
        },
    }
}
