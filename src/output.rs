use polars::prelude::*;
use postgres::{Client, NoTls};
use std::env;
use dotenv::dotenv;

pub fn write_table_out(df: &DataFrame, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let conn_str = env::var("PSQL_STR")?;

    let mut client = Client::connect(&conn_str, NoTls)?;

    // Verify connection by running a simple query
    client.simple_query("SELECT 1").map_err(|e| {
        eprintln!("Failed to connect to the database: {}", e);
        e
    })?;

    // Check if the table exists
    let table_exists = client.query_opt(
        &format!("SELECT 1 FROM information_schema.tables WHERE table_name = '{}'", table_name),
        &[],
    )?.is_some();

    // If the table doesn't exist, create it with the additional columns
    if !table_exists {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id SERIAL PRIMARY KEY,
                {},
                created_at_utc TIMESTAMPTZ DEFAULT NOW(),
                updated_at_utc TIMESTAMPTZ DEFAULT NOW()
            )",
            table_name,
            df.get_columns()
                .iter()
                .map(|s| format!("{} {}", s.name(), sql_type(s.dtype())))
                .collect::<Vec<String>>()
                .join(", ")
        );

        client.batch_execute(&create_table_sql).map_err(|e| {
            eprintln!("Error creating table {}: {}", table_name, e);
            e
        })?;
    }

    let num_rows = df.height();
    let columns = df.get_columns();
    let column_names = columns.iter().map(|c| c.name()).collect::<Vec<_>>().join(", ");
    
    for row_index in 0..num_rows {
        let values = columns
            .iter()
            .map(|col| {
                let value = col.get(row_index).unwrap().to_string();
                format!("'{}'", value.replace("'", "''")) // Escape single quotes in value
            })
            .collect::<Vec<String>>()
            .join(", ");
        
        let insert_sql = format!(
            "INSERT INTO {} (id, {}, created_at_utc, updated_at_utc) VALUES (DEFAULT, {}, NOW(), NOW())",
            table_name, column_names, values
        );

        client.batch_execute(&insert_sql).map_err(|e| {
            eprintln!("Error inserting row {}: {}", row_index, e);
            e
        })?;
    }

    Ok(())
}

// Helper function to map polars data types to SQL types
fn sql_type(dtype: &DataType) -> &'static str {
    match dtype {
        DataType::Int32 | DataType::Int64 => "INTEGER",
        DataType::Float32 | DataType::Float64 => "REAL",
        DataType::Utf8 => "TEXT",
        _ => "TEXT", // Default to TEXT for unsupported types
    }
}
