use rusqlite::{Connection, Error, Result, params};
use tss_esapi::{Context, TctiNameConf, tcti_ldr::{DeviceConfig, TabrmdConfig, TctiInfo}};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {

    let conn = Connection::open("./test.db3")?;

    // conn.pragma_update(None, "key", "test_encryption_key");
    conn.pragma_update(None, "key", "test_encryption_key");

    // conn.execute(
    //     "CREATE TABLE person (
    //         id    INTEGER PRIMARY KEY,
    //         name  TEXT NOT NULL,
    //         data  BLOB
    //     )",
    //     [], // empty list of parameters.
    // )?;

    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };

    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     params![me.name, &me.data],
    // )?;

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

    //tpm
    // match TctiInfo::get_info(TctiNameConf::Device(DeviceConfig::default())) {
    //     Ok(info) => {
    //         eprintln!("info: {}; {}; {};", info.version(), info.name().to_str()?, info.description().to_str()?);
    //     },
    //     Err(error) => {
    //         eprintln!("Error: {}", error);
    //     }
    // }

    let context = match Context::new(TctiNameConf::Device(DeviceConfig::default())) {
        Ok(context) => context,
        Err(error) => {
            eprintln!("Error: {}", error);
            return Err(Error::InvalidQuery);
        }
    };



    Ok(())

}
