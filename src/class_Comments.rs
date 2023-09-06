#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use mysql::prelude::{FromRow, Queryable};
use mysql::{FromRowError, PooledConn, Row};
use serde::Deserialize;
use text_colorizer::Colorize;

use crate::traits;

#[derive(Clone,Debug,PartialEq,Deserialize)]
#[allow(non_camel_case_types)]
pub struct classComment
{
	idComment :u32,
	idMatch :u32,
	MatchComment:String,
}

impl FromRow for classComment
{
	fn from_row_opt(row: Row) -> Result<Self, FromRowError> where Self: Sized
	{
		let tmp :classComment = classComment
		{
			idComment: row.get::<u32, usize>(0).unwrap(),
			idMatch: row.get::<u32, usize>(1).unwrap(),
			MatchComment: row.get::<String,usize>(2).unwrap(),
		};
		Ok(tmp)
	}
}

impl traits::UEPObject for classComment
{
	fn getID(&self) -> u32
	{
		self.idComment
	}

	fn getString(&self) -> String
	{
		self.MatchComment.clone()
	}

	fn getMatchID(&self) -> u32
	{
		self.idMatch
	}

	fn getColor(&self) -> u8
	{
		0
	}

	fn getScoreP(&self) -> u128
	{
		0
	}

	fn getScoreO(&self) -> u128
	{
		0
	}

	fn getVictories(&self) -> u32
	{
		0
	}

	fn getConcedes(&self) -> u32
	{
		0
	}

	fn getDefeats(&self) -> u32
	{
		0
	}

	fn getDraws(&self) -> u32
	{
		0
	}

	fn getMatches(&self) -> u32
	{
		0
	}

	fn getPlayerID(&self) -> u32
	{
		0
	}

	fn getTurns(&self) -> u32
	{
		0
	}

	fn getResult(&self) -> u8
	{
		0
	}

	fn getStartTime(&self) -> u64
	{
		0
	}

	fn getEndTime(&self) -> u64
	{
		0
	}

	fn getManasAmount(&self) -> u8
	{
		0
	}

	fn getMatchDuration(&self) -> u64
	{
		0
	}

	fn getLvlP(&self) -> u32
	{
		0
	}

	fn getLvlO(&self) -> u32
	{
		0
	}

	fn getFromDB(&mut self, connection: &mut PooledConn, id: u32) -> Self
	{
		let mysqlquery = String::from("SELECT * FROM Comments WHERE idComment = <PHid>").replace("<PHid>",id.to_string().as_str());
		let results = connection.query::<classComment,String>(mysqlquery);
		if results.is_err()
		{
			self.idComment=id;
			self.idMatch=0;
			self.MatchComment =String::from("NOT FOUND");
			self.to_owned()
		}
		else
		{
			*self=classComment::from(results.unwrap().get(0).unwrap().clone());
			self.to_owned()
		}
	}

	fn updateme(self, connection: &mut PooledConn) -> bool
	{
		let mysqlquery = String::from
			(
				"UPDATE Comments SET idMatch = <PHidMatch>, MatchComment = '<PHComment>' WHERE idComment=<PHid>"
					.replace("<PHidMatch>",self.idMatch.to_string().as_str())
					.replace("<PHComment>",self.MatchComment.as_str())
					.replace("<PHid>",self.idComment.to_string().as_str())
					.replace("\'","\\'")
			);
		let results = connection.query::<classComment,String>(mysqlquery);
		if results.is_err()
		{
			eprintln!("[{}] -> {}","ERROR".red().bold().blink(),results.as_ref().unwrap_err().to_string());
		}
		results.is_ok()
	}

	fn insertme(self, connection: &mut PooledConn) -> u32
	{
		let mut mysqlquery = String::from
			(
				"INSERT INTO Comments (idMatch, MatchComment) VALUES (<PHidm>,'<PHcom>')"
					.replace("<PHidm>",self.idMatch.to_string().as_str())
					.replace("<PHcom>",&self.MatchComment.clone())
					.replace("\'","\\'")
			);
		let results = connection.query::<classComment,String>(mysqlquery);
		if results.is_err()
		{
			eprintln!("[{}] -> {}","ERROR".red().bold().blink(),results.as_ref().unwrap_err().to_string());
			0
		}
		else
		{
			mysqlquery = String::from("SELECT MAX(idComment) FROM Comments");
			let newid = connection.query::<u32,String>(mysqlquery);
			if newid.is_ok()
			{
				*newid.unwrap().get(0).unwrap()
			}
			else
			{
				eprintln!("[{}] -> {}","ERROR".red().bold().blink(),results.as_ref().unwrap_err().to_string());
				0
			}
		}
	}
}

impl classComment
{
	fn new(&self) -> classComment
	{
		classComment
		{
			idComment: 0,
			idMatch: 0,
			MatchComment: "".to_string()
		}
	}
}


