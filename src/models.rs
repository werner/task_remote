use diesel::*;
use schema::tasks;
use schema::servers;

#[derive(Queryable, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub command: Option<String>,
    pub code: String,
    pub output: Option<String>,
    pub language: Option<String>,
}

#[derive(Insertable, Clone)]
#[table_name="tasks"]
pub struct MutTask {
    pub title: String,
    pub command: Option<String>,
    pub code: String,
    pub output: Option<String>,
    pub language: Option<String>,
}

impl MutTask {
    pub fn new(title: String, command: Option<String>, code: String, output: Option<String>, language: Option<String>) -> MutTask {
        MutTask {
            title: title,
            command: command,
            code: code,
            output: output,
            language: language
        }
    }

    pub fn create(&self, conn: &SqliteConnection) {
        match insert_into(tasks::table)
                .values(self)
                .execute(conn) {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("{}", error)
                }
    }

    pub fn find(conn: &SqliteConnection, id: i32) -> Task {
        tasks::table.find(id).first::<Task>(conn).expect("Not found")
    }
}

#[derive(Queryable, Clone)]
pub struct Language {
    pub id: i32,
    pub name: String,
    pub value: String
}

#[derive(Queryable, Clone)]
pub struct Server {
    pub id: i32,
    pub user: String,
    pub domain_name: String
}

#[derive(Insertable, Clone)]
#[table_name="servers"]
pub struct MutServer {
    pub user: String,
    pub domain_name: String
}

impl MutServer {
    pub fn new(user: String, domain_name: String) -> MutServer {
        MutServer {
            user: user,
            domain_name: domain_name
        }
    }

    pub fn create(&self, conn: &SqliteConnection) {
        match insert_into(servers::table)
                  .values(self)
                  .execute(conn) {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("{}", error)
                  }
    }

    pub fn find(conn: &SqliteConnection, id: i32) -> Server {
        servers::table.find(id).first::<Server>(conn).expect("Not found")
    }
}