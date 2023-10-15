extern crate rusqlite;
use rusqlite::Connection;
use crate::pentry::ServiceInfo;

pub fn init_database() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("passwords.db")?; // You can change the database filename

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            service TEXT,
            username TEXT,
            encrypted_password TEXT
        )",
        [],
    )?;

    Ok(conn)
}

pub fn write_password_to_db(
    conn: &Connection,
    service: &str,
    username: &str,
    encrypted_password: &str,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO passwords (service, username, encrypted_password) VALUES (?, ?, ?)",
        &[&service, &username, &encrypted_password],
    )?;
    Ok(())
}

pub fn read_passwords_from_db(conn: &Connection) -> Result<Vec<ServiceInfo>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT service, username, encrypted_password FROM passwords")?;
    let entries = stmt
        .query_map([], |row| {
            Ok(ServiceInfo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                // &encryption_key,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}

pub fn search_service_by_name(conn: &Connection, name: &str) -> Result<Option<ServiceInfo>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, service, username, encrypted_password FROM passwords WHERE service = ?")?;
    let result = stmt.query_row(&[name], |row| {
        Ok(ServiceInfo {
            id: Some(row.get(0)?),
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}
