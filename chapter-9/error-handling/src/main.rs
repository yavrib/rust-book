fn connect(success: bool) -> Result<String, String> {
    if success {
        return Result::Ok("Success".to_string());
    }

    return Result::Err("Failure".to_string());
}

fn main() {
    let success: Result<String, String> = connect(true);
    let failure: Result<String, String> = connect(false);

    match success {
        Ok(message) => println!("success: {:?}", message),
        Err(error) => panic!("success: {:?}", error),
    }

    match failure {
        Ok(message) => println!("failure: {:?}", message),
        Err(error) => panic!("failure: {:?}", error),
    }
}
