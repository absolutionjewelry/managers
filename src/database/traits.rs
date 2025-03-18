use sqlx::{postgres::PgRow, Error};

pub trait DatabaseResource {
    fn from_row(row: &PgRow) -> Result<Self, Error>
    where
        Self: Sized;
}
