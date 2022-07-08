use serde::{Deserialize, Serialize};
use diesel::pg::PgConnection;
use diesel::{prelude::*, connection};
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post{
  pub id: i32,
  pub title:String,
  pub slug: String,
  pub body: String
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct PostSimplificado{
  pub title:String,
  pub body: String
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub body: &'a str,
  pub slug: &'a str,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
  pub title: String,
  pub body: String
}

impl Post {

  pub fn slugify(title: &str) -> String {
    return title.replace(" ", "-").to_lowercase();
  }
 
  pub fn crate_post<'a>(conn: &PgConnection, post: &NewPostHandler) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title.clone());

        let new_post = NewPost {
        title: &post.title,
        body: &post.body,
        slug: &slug
    };

    diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn)
  }
}