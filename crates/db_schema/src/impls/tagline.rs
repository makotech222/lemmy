use crate::{
  source::tagline::{Tagline, TaglineForm},
  traits::Crud,
};
use diesel::{dsl::*, result::Error, PgConnection, QueryDsl, RunQueryDsl};

impl Crud for Tagline {
  type Form = TaglineForm;
  type IdType = i32;
  fn create(conn: &PgConnection, new_tagline: &TaglineForm) -> Result<Self, Error> {
    use crate::schema::tagline::dsl::*;
    insert_into(tagline)
      .values(new_tagline)
      .get_result::<Self>(conn)
  }

  fn read(conn: &PgConnection, tagline_id: i32) -> Result<Self, Error> {
    use crate::schema::tagline::dsl::*;
    tagline.find(tagline_id).first::<Self>(conn)
  }

  fn update(
    conn: &PgConnection,
    tagline_id: i32,
    new_tagline: &TaglineForm,
  ) -> Result<Self, Error> {
    use crate::schema::tagline::dsl::*;
    diesel::update(tagline.find(tagline_id))
      .set(new_tagline)
      .get_result::<Self>(conn)
  }

  fn delete(conn: &PgConnection, tagline_id: i32) -> Result<usize, Error> {
    use crate::schema::tagline::dsl::*;
    diesel::delete(tagline.find(tagline_id)).execute(conn)
  }
}

impl Tagline {
  pub fn get_random(conn: &PgConnection) -> Result<Self, Error> {
    use crate::schema::tagline::dsl::*;
    no_arg_sql_function!(
      random,
      diesel::sql_types::Serial,
      "Represents the SQL random() function"
    );
    tagline.order(random).first::<Self>(conn)
  }

  pub fn get_all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
    use crate::schema::tagline::dsl::*;
    tagline.load::<Self>(conn)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    source::tagline::{Tagline, TaglineForm},
    utils::establish_unpooled_connection,
  };

  #[test]
  fn test_crud() {
    let conn = establish_unpooled_connection();

    let tagline_text = "Example tagline".to_string();

    let tagline_form = TaglineForm {
      content: tagline_text.clone(),
    };

    // Test create
    let inserted_tagline = Tagline::create(&conn, &tagline_form).unwrap();

    let expected_tagline = Tagline {
      id: inserted_tagline.id,
      content: tagline_text,
    };

    // Test read
    let read_tagline = Tagline::read(&conn, inserted_tagline.id).unwrap();

    // Test update
    let updated_tagline_text = "A new tagline".to_string();
    let updated_tagline_form = TaglineForm {
      content: updated_tagline_text.clone(),
    };
    let updated_tagline =
      Tagline::update(&conn, inserted_tagline.id, &updated_tagline_form).unwrap();

    let expected_updated_tagline = Tagline {
      id: updated_tagline.id,
      content: updated_tagline_text,
    };

    let read_updated_tagline = Tagline::read(&conn, updated_tagline.id).unwrap();

    // Test delete
    Tagline::delete(&conn, inserted_tagline.id).unwrap();

    assert_eq!(expected_tagline, read_tagline);
    assert_eq!(expected_tagline, inserted_tagline);
    assert_eq!(expected_updated_tagline, read_updated_tagline);
    assert_eq!(expected_updated_tagline, updated_tagline);
  }
}
