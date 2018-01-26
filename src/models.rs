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

    fn create(&self, conn: &SqliteConnection) {
        match insert_into(tasks::table)
                .values(self)
                .execute(conn) {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("{}", error)
                }
    }

    fn update(&self, conn: &SqliteConnection, id: i32) {
        use schema::tasks::dsl::{tasks, title, command, code, output, language};
        let _command = self.command.clone();
        let _output = self.output.clone();
        let _language = self.language.clone();
        match update(tasks.find(id))
                  .set((title.eq(&self.title),
                        command.eq(_command.unwrap_or(String::new())),
                        code.eq(&self.code),
                        output.eq(_output.unwrap_or(String::new())),
                        language.eq(_language.unwrap_or(String::new()))))
                  .execute(conn) {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("{}", error)
                  }
    }

    pub fn find(conn: &SqliteConnection, id: i32) -> Task {
        tasks::table.find(id).first::<Task>(conn).expect("Not found")
    }

    pub fn save(&self, conn: &SqliteConnection, id: i32) {
        if id > 0 {
            self.update(conn, id);
        } else {
            self.create(conn);
        }
    }
}

#[derive(Queryable, Clone)]
pub struct Language {
    pub id: i32,
    pub name: String,
    pub value: String
}

#[derive(Queryable, Clone, Debug)]
pub struct Server {
    pub id: i32,
    pub user: String,
    pub domain_name: String
}

#[derive(Insertable, Clone, Debug)]
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

    fn update(&self, conn: &SqliteConnection, id: i32) {
        use schema::servers::dsl::{servers, user, domain_name};
        match update(servers.find(id))
                  .set((user.eq(&self.user), domain_name.eq(&self.domain_name)))
                  .execute(conn) {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("{}", error)
                  }
    }

    pub fn find(conn: &SqliteConnection, id: i32) -> Result<Server, result::Error> {
        servers::table.find(id).first::<Server>(conn)
    }

    pub fn save(&self, conn: &SqliteConnection, id: i32) {
        if id > 0 {
            self.update(conn, id);
        } else {
            self.create(conn);
        }
    }
}