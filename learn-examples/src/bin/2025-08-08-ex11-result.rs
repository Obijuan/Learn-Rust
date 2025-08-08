fn main()
{
    let a: Result<u8,String> = Ok(5);
    let b: Result<u8,String> = Err("Error".to_string());

    match a {
        Ok(value) => {
            println!("Valor de a: {}", value);
        }
        Err(msg) => {
            println!("Error: {}", msg);
        }
    }
    

    match b {
        Ok(value) => {
            println!("Valor de b: {}", value);
        }
        Err(msg) => {
            println!("Valor de b: {}", msg);
        }
    }
}
