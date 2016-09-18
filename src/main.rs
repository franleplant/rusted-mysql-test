#[macro_use]
extern crate mysql;

use mysql as my;
use std::fmt;
use mysql::value::from_row;


fn main() {
    let pool = my::Pool::new("mysql://root:root@localhost:3306").unwrap();

    let mut stmt = pool.prepare("SELECT ?").unwrap();

    for row in stmt.execute(("@@slow_query_log",)).unwrap() {
        println!("{}", row.unwrap());
        //let cell = from_row(row.unwrap());
        //println!("{}", cell)
    }


}

