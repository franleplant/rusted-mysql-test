#[macro_use]
extern crate mysql;

use mysql as my;
use mysql::value::from_row;


fn main() {
    let pool = my::Pool::new("mysql://root:root@localhost:3306").unwrap();

    let mut stmt = pool.prepare("SELECT ?").unwrap();

    for row in stmt.execute(("@@slow_query_log",)).unwrap() {
        let res = row.unwrap();
        // Print the raw row
        println!("RAW ROW: {:?}", res);
        // ::<Type> should match the type of params in the `execute`
        // it tells Rust the type of the tuple you are getting from the row
        let row_tuple = from_row::<String>(res);
        println!("TUPPLE ROW: {}", row_tuple);
    }


}

