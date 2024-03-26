use serde::Serialize;

#[derive(Serialize)]
pub struct Tweet {
    pub id: String,
    pub body: String,
}
