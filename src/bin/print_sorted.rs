use postgres::{Client, NoTls, Error};

struct Nation {
    country: String,
    count: i64,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://dboperator:operatorpass123@localhost:5243/postgres", NoTls)?;
    
    for row in client.query 
    ("SELECT country, COUNT(country) AS count 
    FROM author GROUP BY country ORDER BY count DESC", &[])? {
        
        let (country, count) : (Option<String>, Option<i64>) 
        = (row.get (0), row.get (1));
        
        if country.is_some () && count.is_some () {

            let nation = Nation{
                country: country.unwrap(),
                count: count.unwrap(),
        };
            println!("{} {}", nation.country, nation.count);
            
        }
    }
    
    Ok(())

}
