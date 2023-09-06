#![allow(dead_code)]

use dotenvy::dotenv;
use mysql::{Error, Pool, PooledConn};
use crate::{traits};

pub fn establish_connection() -> Result<PooledConn,Error>
{
	dotenvy::from_path("/2023/Programmation/Rust/uepMagicStatsServer/.env").expect("oups caca");
	dotenv().ok();

	let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let pool = Pool::new(database_url.as_str())?;
	let conn = pool.get_conn()?;
	Ok(conn)
}

#[allow(non_camel_case_types)]
pub fn GEN_updateDB<gentype>(object :gentype, connection :&mut PooledConn) -> bool where gentype: traits::UEPObject
{
	object.updateme(connection)
}

#[allow(non_camel_case_types)]
pub fn GEN_insertDB<gentype>(object :gentype, connection :&mut PooledConn) -> u32 where gentype: traits::UEPObject
{
	let newid=object.insertme(connection);
	newid
}
