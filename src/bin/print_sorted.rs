use postgres::{Client, NoTls, Error};

struct Sorted {
    catalogue: String,
    id: i64,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://dboperator:operatorpass123@localhost:5243/postgres", NoTls)?;
    
    for row in client.query 
    ("SELECT * FROM b_store", &[])? {
        
        let (item, id) : (Option<String>, Option<i64>) 
        = (row.get (0), row.get (1));
        
        if item.is_some () && id.is_some () {

            let sorted_db = Sorted {
                catalogue: item.unwrap(),
                id: id.unwrap(),
        };
            println!("{} {}", sorted_db.catalogue, sorted_db.id);
            
        }
    }
    
    // let sorted = client.query("SELECT * FROM b_store", &[]);
    // println!("{:?}", sorted);
    Ok(())

}
