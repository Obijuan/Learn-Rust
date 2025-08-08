fn id(a: Result<u8, &str>) -> Result<u8, &str> 
{
    let c = a?;

    println!("Valor: {}", c);
    return Ok(c);
}

fn main()
{
    let a: Result<u8, &str> = Ok(5);
    let b: Result<u8, &str> = Err("Error");

    id(a).expect("Panic!");
    id(b).expect("panic!");
}
