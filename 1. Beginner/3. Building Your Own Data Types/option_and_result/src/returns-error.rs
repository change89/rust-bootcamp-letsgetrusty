fn main() {
    let empty_query = String::new();
    let username = get_username(empty_query);
    if let Some(name) = username {
        println!("{}", name);
    } else {
        println!("Error: Query string is empty!");
    }
}

fn get_username(query: String) -> Option<String> {
    let db_result = query_db(query);
    // Here we will return Option result to capture any error if occurs
    // because Option is a special enum with 2 variants Ok and None
    // we also can use Result here
    match db_result {
        Ok(value) => Some(value),
        Err(error) => {
            println!("Error: {}", error);
            None
        }
    }
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty!"))
    } else {
        Ok(String::from("Ferris"))
    }
}
