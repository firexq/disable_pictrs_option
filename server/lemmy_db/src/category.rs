use crate::{
  schema::{category, category::dsl::*},
  Crud,
};
use diesel::{dsl::*, result::Error, *};
use serde::{Serialize};

#[derive(Queryable, Identifiable, PartialEq, Debug, Serialize)]
#[table_name = "category"]
pub struct Category {
  pub id: i32,
  pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "category"]
pub struct CategoryForm {
  pub name: String,
}

impl Crud<CategoryForm> for Category {
  fn read(conn: &PgConnection, category_id: i32) -> Result<Self, Error> {
    category.find(category_id).first::<Self>(conn)
  }

  fn create(conn: &PgConnection, new_category: &CategoryForm) -> Result<Self, Error> {
    insert_into(category)
      .values(new_category)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &PgConnection,
    category_id: i32,
    new_category: &CategoryForm,
  ) -> Result<Self, Error> {
    diesel::update(category.find(category_id))
      .set(new_category)
      .get_result::<Self>(conn)
  }
}

impl Category {
  pub fn list_all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
    category.load::<Self>(conn)
  }
}

#[cfg(test)]
mod tests {
  use crate::{category::Category, tests::establish_unpooled_connection};

  #[test]
  fn test_crud() {
    let conn = establish_unpooled_connection();

    let categories = Category::list_all(&conn).unwrap();
    let expected_first_category = Category {
      id: 1,
      name: "Discussion".into(),
    };

    assert_eq!(expected_first_category, categories[0]);
  }
}
