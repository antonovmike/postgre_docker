https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html

https://tms-dev-blog.com/postgresql-database-with-rust-how-to/

create_db.rs

insert.rs

print_sorted.rs

## Start and run

You have to have docker and docker-compose installed to be able to use the docker-compose.yml.

To start the PostgreSQL instance:

```bash
sudo docker ps
sudo docker-compose up -d
sudo docker-compose up
```

To start the program:

```bash
cargo run
```

```bash
sudo docker exec -ti rust-postgresql-tutorial_postgres_1 bash

root@c97ee6471bc4:/# psql -d postgres -U dboperator

postgres=# \l
```

