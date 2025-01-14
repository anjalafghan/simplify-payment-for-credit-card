use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {

        let conn = rusqlite::Connection::open("credit_cards.db").unwrap_or_else(|err| {
            eprintln!("Failed to open database: {}", err);
            panic!("Database initialization failed");
        });

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY,
                card_name TEXT NOT NULL
            );
            "#,
        ).unwrap_or_else(|err| {
            eprintln!("Failed to create table: {}", err);
            panic!("Table creation failed");
        });

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS transactions(
                id INTEGER PRIMARY KEY,
                card_id INTEGER NOT NULL,
                amount REAL NOT NULL,
                FOREIGN KEY (card_id) REFERENCES cards(id)
            );
            "#,
        ).unwrap_or_else(|err| {
            eprintln!("Failed to create table: {}", err);
            panic!("Table creation failed");
        });
        conn
    };
}

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|db| {
        if let Err(err) = db.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]) {
            eprint!("Failed to save dog: {}", err);
            return Err(ServerFnError::new(err.to_string()));
        }

        Ok(())
    });

    Ok(())
}

// Query the database and return the last 10 dogs and their url
#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[server]
pub async fn delete_dog(id: usize) -> Result<(), ServerFnError> {
    DB.with(|db| {
        if let Err(err) = db.execute("DELETE FROM dogs WHERE id = ?1", &[&id]) {
            eprint!("Failed to delete dog: {}", err);
            return Err(ServerFnError::new(err.to_string()));
        }

        Ok(())
    });

    Ok(())
}
#[server]
pub async fn list_cards() -> Result<Vec<(usize, String)>, ServerFnError> {
    let cards = DB.with(|f| {
        f.prepare("SELECT id, card_name FROM cards")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(cards)
}

#[server]
pub async fn save_card(name: String) -> Result<(), ServerFnError> {
    DB.with(|db| {
        if let Err(err) = db.execute("INSERT INTO cards (card_name) VALUES (?1)", &[&name]) {
            tracing::error!("Failed to save card: {}", err);
            return Err(ServerFnError::new(format!("Database error: {}", err)));
        }
        tracing::info!("Successfully saved card with name: {:?}", name);
        Ok(())
    })
}

#[server]
pub async fn save_transaction(card_id: usize, transaction: f64) -> Result<(), ServerFnError> {
    DB.with(|db| {
        let card_id = card_id as i64;
        let transactions = transaction as f64;
        if let Err(err) = db.execute(
            "INSERT INTO transactions (card_id, amount) VALUES (?1, ?2)",
            rusqlite::params![&card_id, &transactions],
        ) {
            tracing::error!("Failed to save transaction: {}", err);
            return Err(ServerFnError::new(format!("Database error: {}", err)));
        }
        tracing::info!(
            "Successfully saved transaction to card with id: {:?}",
            card_id
        );
        Ok(())
    })
}

#[server]
pub async fn get_transactions(card_id: usize) -> Result<f64, ServerFnError> {
    let amount: f64 = DB.with(|f| {
        f.prepare("SELECT SUM(amount) FROM transactions WHERE card_id = ?1")
            .and_then(|mut stmt| {
                stmt.query_row([card_id], |row| row.get(0)) // Get the single result (SUM(amount))
            })
            .unwrap_or(0.0) // Default to 0.0 if there's an error or no result
    });
    Ok(amount)
}
