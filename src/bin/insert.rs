use postgres::{Client, NoTls, Error};

// struct BStore {
//     _id: i32,
//     cat: String,
//     item: String,
//     price: i32
// }

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://dboperator:operatorpass123@localhost:5243/postgres", NoTls)?;
    
    client.execute(
        "INSERT INTO b_store VALUES (1, 'sport', 'helmet', 202)", &[]
    )?;

    Ok(())

}

