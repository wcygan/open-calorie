use limbo::{Builder, Value}; // Added Value to imports for easier matching
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to or create a database file named "sqlite.db"
    let db = Builder::new_local("sqlite.db").build().await?;
    let conn = db.connect()?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)",
        (),
    )
    .await?;

    // Insert data
    conn.execute("INSERT INTO users (id, name) VALUES (1, 'Eve')", ())
        .await?;
    conn.execute("INSERT INTO users (id, name) VALUES (2, 'Mallory')", ())
        .await?;

    // Retrieve data
    let mut stmt = conn.prepare("SELECT id, name FROM users").await?;
    let mut rows = stmt.query(()).await?; // Execute the query asynchronously

    while let Some(row) = rows.next().await? {
        let id_value = row.get_value(0)?;
        let id: i32 = match id_value {
            Value::Integer(val) => val as i32, // Assuming the DB integer fits in i32
            other => {
                return Err(format!(
                    "Expected Integer for id, found {:?} at index 0",
                    other
                )
                .into())
            }
        };

        let name_value = row.get_value(1)?;
        let name: String = match name_value {
            Value::Text(val) => val,
            other => {
                return Err(format!(
                    "Expected Text for name, found {:?} at index 1",
                    other
                )
                .into())
            }
        };
        println!("User: ID: {}, Name: {}", id, name);
    }

    Ok(())
}