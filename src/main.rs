use rusqlite::{Connection, Result};
use rusqlite::types::{FromSql, ValueRef};

#[derive(Debug)]
enum Status {Applied, Rejected, HRInterview, EngInterview,Offer,OfferAccepted}

impl ToSQL for Status {
    fn to_sql(&self) -> &'static str {}
}

impl FromSql for Status {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {}
}

//Find an enum for date

struct Application {
    date: String,
    company_name: String,
    position_name: String,
    location: String,
    status: Status,
    extra_info: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE application (
            date    INTEGER PRIMARY KEY,
            company_name  TEXT NOT NULL,
            position_name  TEXT NOT NULL,
            location      TEXT NOT NULL,
            status TEXT NOT NULL,
            extra_info  TEXT,
        )",
        (), // empty list of parameters.
    )?;
    let me = Application {
        date: "Steven".to_string(),
        company_name: "Steven".to_string(),
        position_name: "Steven".to_string(),
        location: "Steven".to_string(),
        status: Status::HRInterview,
        extra_info: "chickens".to_string(),
    };
    conn.execute(
        "INSERT INTO application (date, company_name, position_name, location, status, extra_info) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&me.date, &me.company_name, &me.position_name, &me.location,&me.status, &me.extra_info),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}