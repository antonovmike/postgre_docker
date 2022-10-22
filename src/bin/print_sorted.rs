use postgres::{Client, NoTls, Error};

struct Sorted {
    item: String,
    price: i64,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres", NoTls
    )?;
    
    for row in client.query 
    ("SELECT item, COUNT(item) AS goods 
    FROM b_store GROUP BY id ORDER BY goods DESC", &[])? {
        let (item, price): (Option<String>, Option<i64>) = (row.get(0), row.get(1));
        
        if item.is_some() && price.is_some() {
            let sorted_db = Sorted {
                item: item.unwrap(),
                price: price.unwrap(),
        };
            println!("{} {}", sorted_db.item, sorted_db.price);
            
        }
    }
    
    Ok(())
}
