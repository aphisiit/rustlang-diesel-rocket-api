use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::mysql::MysqlConnection;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn connect() -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}