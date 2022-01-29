use crate::schema::tagline;
use serde::{Deserialize, Serialize};
#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "tagline"]
pub struct Tagline {
  pub id: i32,
  pub content: String,
}

#[derive(Insertable, AsChangeset, Default)]
#[table_name = "tagline"]
pub struct TaglineForm {
  pub content: String,
}
