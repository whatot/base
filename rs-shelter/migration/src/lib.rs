use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, Statement};

pub async fn up(conn: &DatabaseConnection) -> Result<(), DbErr> {
    conn.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        r#"
        CREATE TABLE IF NOT EXISTS dog (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            description TEXT NOT NULL,
            date_of_birth DATE NOT NULL,
            date_of_vaccination DATE,
            chip_number VARCHAR NOT NULL,
            gender VARCHAR NOT NULL,
            is_sterilized BOOLEAN NOT NULL,
            breed VARCHAR NOT NULL,
            size VARCHAR NOT NULL,
            weight INTEGER,
            hair VARCHAR NOT NULL
        )
        "#,
    ))
    .await?;

    conn.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        r#"
        CREATE TABLE IF NOT EXISTS "user" (
            id SERIAL PRIMARY KEY,
            username VARCHAR NOT NULL,
            password VARCHAR NOT NULL
        )
        "#,
    ))
    .await?;

    Ok(())
}
