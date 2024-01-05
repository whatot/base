use anyhow::anyhow;
use argon2::Argon2;
use clap::{Arg, ArgMatches, Command};
use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Database, EntityTrait, QueryFilter, Set,
};

use crate::settings::Settings;

pub fn configure() -> Command {
    Command::new("create_admin")
        .about("Create the default admin user")
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Password for admin user")
                .default_value("Pa$$wd123"),
        )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("create_admin") {
        let password = matches.get_one::<String>("password").unwrap();
        let encrypted_password = encrypt_password(&password)?;

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.get_db_url();
                let conn = Database::connect(db_url)
                    .await
                    .expect("Database connection failed");

                let admin: Option<entity::user::Model> = entity::user::Entity::find()
                    .filter(entity::user::Column::Username.eq("admin"))
                    .one(&conn)
                    .await?;

                if admin.is_some() {
                    println!("Admin user already exists");
                    return anyhow::Ok(());
                }

                let admin_model = entity::user::ActiveModel {
                    id: NotSet,
                    username: Set("admin".to_owned()),
                    password: Set(encrypted_password.to_owned()),
                };

                if let Ok(_admin) = admin_model.save(&conn).await {
                    println!("Admin user created");
                } else {
                    println!("Failed to create admin user");
                }

                return anyhow::Ok(());
            })?
    }

    anyhow::Ok(())
}

fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    if let Ok(hash) = argon2.hash_password(password.as_bytes(), &salt) {
        anyhow::Ok(hash.to_string())
    } else {
        Err(anyhow!("Failed to hash password"))
    }
}
