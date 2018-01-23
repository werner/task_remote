#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub pre_hook: Option<String>,
    pub code: String,
    pub post_hook: Option<String>,
    pub language: Option<String>,
}