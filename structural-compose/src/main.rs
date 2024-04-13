mod example_issue {
    struct Database {
        connection_string: String,
        timeout: u32,
        pool_size: u32,
    }

    fn print_database(database: &Database) {
        println!("Connection string: {}", database.connection_string);
        println!("Timeout: {}", database.timeout);
        println!("Pool size: {}", database.pool_size);
    }

    fn main_example() {
        let mut db = Database {
            connection_string: "init string".to_string(),
            timeout: 30,
            pool_size: 10,
        };
        let connection_string = &mut db.connection_string;
        print_database(&db); // Immutable borrow of `db` happens here
                             // *connection_string = "new string".to_string();  // Mutable borrow is used
    }
}

#[derive(Debug, Clone)]
struct ConnectionString(String);

#[derive(Debug, Clone, Copy)]
struct Timeout(u32);

#[derive(Debug, Clone, Copy)]
struct PoolSize(u32);

struct Database {
    connection_string: ConnectionString,
    timeout: Timeout,
    pool_size: PoolSize,
}

fn print_database(connect_str: ConnectionString, timeout: Timeout, pool_size: PoolSize) {
    println!("Connection string: {:?}", connect_str);
    println!("Timeout: {:?}", timeout);
    println!("Pool size: {:?}", pool_size);
}

fn main() {
    let mut db = Database {
        connection_string: ConnectionString("init string".to_string()),
        timeout: Timeout(30),
        pool_size: PoolSize(10),
    };

    let con_string = &mut db.connection_string;
    print_database(con_string.clone(), db.timeout, db.pool_size);
    *con_string = ConnectionString("new string".to_string());
}
