use enterprise_module_lib::adapter::config::app_settings::AppSettings;
use postgres::{Client, Error, NoTls};

pub struct PostgresDatabase {
    pub database_url: String,
}

impl PostgresDatabase {
    pub fn new(app_settings: &AppSettings) -> Self {
        Self {
            database_url: app_settings.database_url.clone(),
        }
    }

    pub fn start(&self) -> Result<(), Error> {
        println!("database_url: {}", &self.database_url);

        let mut client = Client::connect(&self.database_url, NoTls)?;
        client.batch_execute(
            "
            CREATE TABLE IF NOT EXISTS client (id uuid, first_name text, last_name text, email text);
            CREATE TABLE IF NOT EXISTS barber (id uuid, first_name text, last_name text, email text);
            CREATE TABLE IF NOT EXISTS task (id uuid, description text, price money, duration_in_minutes int);
            CREATE TABLE IF NOT EXISTS appointment (
                id uuid,
                start_at timestamp,
                end_at timestamp,
                client_id uuid, 
                barber_id uuid,
                task_id uuid
            );

            insert into client (id, first_name, last_name, email) values ('c85434b2-7b11-43b8-b7d4-d9d293756d65', 'DANILO', 'ARRUDA', 'DANILO@EMAIL.COM');
            insert into barber (id, first_name, last_name, email) values ('c85434b2-7b11-43b8-b7d4-d9d293756d65', 'DENILSON', 'ARRUDA', 'DANILO@EMAIL.COM');
            insert into task (id, description, price, duration_in_minutes) values ('c85434b2-7b11-43b8-b7d4-d9d293756d65', 'MOICANO', 50.00, 40);
            ",
        )?;
        Ok(())
    }
}
