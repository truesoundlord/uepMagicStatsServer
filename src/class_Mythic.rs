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
pub struct classMythic
{
	idMythic :u32,
	idMatch :u32,
	PourcentageEn :u8,
	PourcentagePl :u8,
	TopEn :u16,
	TopPl :u16
}

impl FromRow for classMythic
{
	fn from_row_opt(row: Row) -> Result<Self, FromRowError> where Self: Sized
	{
		let tmp :classMythic = classMythic
		{
			idMythic: row.get::<u32, usize>(0).unwrap(),
			idMatch: row.get::<u32, usize>(1).unwrap(),
			PourcentageEn: row.get::<u8,usize>(2).unwrap(),
			PourcentagePl: row.get::<u8,usize>(3).unwrap(),
			TopEn: row.get::<u16,usize>(4).unwrap(),
			TopPl: row.get::<u16,usize>(5).unwrap()
		};
		Ok(tmp)
	}
}

impl traits::UEPObject for classMythic
{
	fn getID(&self) -> u32
	{
		self.idMythic
	}

	fn getString(&self) -> String
	{
		"".to_string()
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
		if self.TopPl == 0
		{
			self.PourcentagePl as u32
		}
		else
		{
			self.TopPl as u32
		}
	}

	fn getLvlO(&self) -> u32
	{
		if self.TopEn == 0
		{
			self.PourcentageEn as u32
		}
		else
		{
			self.TopEn as u32
		}
	}

	fn getFromDB(&mut self, connection: &mut PooledConn, id: u32) -> Self
	{
		let mysqlquery = String::from("SELECT * FROM Matches WHERE idMatch = <PHid>").replace("<PHid>",id.to_string().as_str());
		let results = connection.query::<classMythic,String>(mysqlquery);
		if results.is_ok()
		{
			*self=classMythic::from(results.unwrap().get(0).unwrap().clone());
			self.to_owned()
		}
		else
		{
			self.to_owned()
		}
	}

	fn updateme(self, connection: &mut PooledConn) -> bool
	{
		let mysqlquery = String::from
			(
				"UPDATE Comments SET idMatch = <PHidMatch>,
						PourcentageEn = <PHPerEn>,
						PourcentagePl = <PHPerPl>,
						TopEn = <PHTEn>,
						TopPl = <PHTPl>
						WHERE idMythic=<PHid>"
					.replace("<PHidMatch>",self.idMatch.to_string().as_str())
					.replace("<PHPerEn>",self.PourcentageEn.to_string().as_str())
					.replace("<PHPerPl>",self.PourcentagePl.to_string().as_str())
					.replace("<PHTEn>",self.TopEn.to_string().as_str())
					.replace("<PHTPl>",self.TopPl.to_string().as_str())
					.replace("<PHid>",self.idMythic.to_string().as_str())
			);
		let results = connection.query::<classMythic,String>(mysqlquery);
		if results.is_err()
		{
			eprintln!("[{}] -> {}","ERROR".red().bold().blink(),results.as_ref().unwrap_err().to_string());
		}
		results.is_ok()
	}

	fn insertme(self, connection: &mut PooledConn) -> u32 {
		todo!()
	}
}

impl classMythic
{
	fn new(&self) -> classMythic
	{
		classMythic
		{
			idMythic: 0,
			idMatch: 0,
			PourcentageEn: 0,
			PourcentagePl: 0,
			TopEn: 0,
			TopPl: 0,
		}
	}
}
