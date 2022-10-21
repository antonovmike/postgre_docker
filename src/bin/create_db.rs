// sudo docker exec -ti postgre_docker_postgres_1 bash

use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres", NoTls
    )?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS b_store (
            id              SERIAL PRIMARY KEY,
            cat             VARCHAR NOT NULL,
            item            VARCHAR NOT NULL,
            price           INT
            )
    ")?;

    Ok(())
}

