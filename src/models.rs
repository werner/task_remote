use diesel::*;
use schema::tasks;

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

    pub fn create(&self, conn: &SqliteConnection) -> usize {
        insert_into(tasks::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new post")
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