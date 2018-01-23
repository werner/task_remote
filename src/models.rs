use diesel::*;
use schema::tasks;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub pre_hook: Option<String>,
    pub code: String,
    pub post_hook: Option<String>,
    pub language: Option<String>,
}

#[derive(Insertable, Clone)]
#[table_name="tasks"]
pub struct MutTask {
    pub title: String,
    pub pre_hook: Option<String>,
    pub code: String,
    pub post_hook: Option<String>,
    pub language: Option<String>,
}

impl MutTask {
    pub fn new(title: String, pre_hook: Option<String>, code: String, post_hook: Option<String>, language: Option<String>) -> MutTask {
        MutTask {
            title: title,
            pre_hook: pre_hook,
            code: code,
            post_hook: post_hook,
            language: language
        }
    }

    pub fn create(&self, conn: &SqliteConnection) -> usize {
        insert_into(tasks::table)
            .values(self)
            .execute(conn)
            .expect("Error saving new post")
    }
}