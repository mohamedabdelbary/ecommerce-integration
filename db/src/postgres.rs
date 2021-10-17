use deadpool_postgres::tokio_postgres::{NoTls, Error, Config, Row};
use deadpool_postgres::{Pool, Manager};
use deadpool_postgres::tokio_postgres::types::ToSql;


pub fn create_pool(host: &str, user: &str, pass: &str, db_name: &str, port: u16, size: usize) -> Pool {
    let mut cfg = Config::new();
    cfg.host(host);
    cfg.user(user);
    cfg.password(pass);
    cfg.dbname(db_name);
    cfg.port(port);
    let mgr = Manager::new(cfg, NoTls);
    Pool::new(mgr, size)
}

pub async fn run_query<T>(query: &str, params_vec: Vec<T>, pool: &Pool) -> Result<Vec<Row>, Error>
    where
        T: ToSql + Sync + Sized
{
    let client = pool.get().await.unwrap();
    let params = get_param_vec::<T>(&params_vec);
    let stmt = client.prepare(query).await.unwrap();
    let results = client.query(&stmt, &params[..]).await.unwrap();
    Ok(results)
}

// See https://stackoverflow.com/questions/46624591/cannot-call-rusqlites-query-because-it-expects-the-type-rusqlitetypestos
// and https://users.rust-lang.org/t/no-matter-what-i-do-i-cannot-satisfy-the-type-checker/40124/19
// for more info.
fn get_param_vec<'a, T: 'a>(params: &'a Vec<T>) -> Vec<&'a (dyn ToSql + Sync)>
    where
        T: ToSql + Sync + Sized
{
    params.iter().map(|x| x as &(dyn ToSql + Sync)).collect()
}
