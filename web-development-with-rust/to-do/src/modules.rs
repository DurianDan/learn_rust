use serde::Serialize;

#[derive(Serialize)] // means that the 'Status' struct is now an `impl` of `Serialize` which means it can be serialized by json()
pub struct Status {
    pub user: String
}