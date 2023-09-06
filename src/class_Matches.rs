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
pub struct classMatches
{
	idMatch :u32,
	idPlayer :u32,
	MatchColor :u8,
	Turns :u32,
	HisScore :u128,
	MyScore :u128,
	Result :u8,
	StartTime :u64,
	EndTime :u64,
	EnLvl :u32,
	MyLvl :u32,
	Manas :u8,
	Duration :u64,
}

impl FromRow for classMatches
{
	fn from_row_opt(row: Row) -> Result<Self, FromRowError> where Self: Sized
	{
		let tmp :classMatches = classMatches
		{
			idMatch: row.get::<u32, usize>(0).unwrap(),
			idPlayer: row.get::<u32, usize>(1).unwrap(),
			MatchColor: row.get::<u8, usize>(2).unwrap(),
			Turns: row.get::<u32, usize>(3).unwrap(),
			HisScore: row.get::<u128, usize>(4).unwrap(),
			MyScore: row.get::<u128, usize>(5).unwrap(),
			Result: row.get::<u8, usize>(6).unwrap(),
			StartTime: row.get::<u64, usize>(7).unwrap(),
			EndTime: row.get::<u64, usize>(8).unwrap(),
			EnLvl: row.get::<u32, usize>(9).unwrap(),
			MyLvl: row.get::<u32, usize>(10).unwrap(),
			Manas: row.get::<u8, usize>(11).unwrap(),
			Duration: row.get::<u64, usize>(12).unwrap()
		};
		Ok(tmp)
	}
}

impl traits::UEPObject for classMatches
{
	fn getID(&self) -> u32
	{
		self.idMatch
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
		self.MatchColor
	}

	fn getScoreP(&self) -> u128
	{
		self.MyScore
	}

	fn getScoreO(&self) -> u128
	{
		self.HisScore
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
		self.idPlayer
	}

	fn getTurns(&self) -> u32
	{
		self.Turns
	}

	fn getResult(&self) -> u8
	{
		self.Result
	}

	fn getStartTime(&self) -> u64
	{
		self.StartTime
	}

	fn getEndTime(&self) -> u64
	{
		self.EndTime
	}

	fn getManasAmount(&self) -> u8
	{
		self.Manas
	}

	fn getMatchDuration(&self) -> u64
	{
		self.Duration
	}

	fn getLvlP(&self) -> u32
	{
		self.MyLvl
	}

	fn getLvlO(&self) -> u32
	{
		self.EnLvl
	}

	fn getFromDB(&mut self, connection: &mut PooledConn, id: u32) -> Self
	{
		let mysqlquery = String::from("SELECT * FROM Matches WHERE idMatch = <PHid>").replace("<PHid>",id.to_string().as_str());
		let results = connection.query::<classMatches,String>(mysqlquery);
		if results.is_ok()
		{
			*self=classMatches::from(results.unwrap().get(0).unwrap().clone());
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
				"UPDATE Matches SET idPlayer = <PHidPlayer>,
						SET MatchColor = <PHMatchColor>,
						Turns = <PHTurns>,
						HisScore = <PHHisScore>,
						MyScore = <PHMyScore>,
						Result = <PHResult>,
						EnLvl = <PHEnLvl>,
						MyLvl = <PHMyLvl>,
						Manas = <PHManas>,
						WHERE idMatch=<PHid>"
					.replace("<PHid>",self.idMatch.to_string().as_str())
					.replace("<PHidPlayer>",self.idPlayer.to_string().as_str())
					.replace("<PHMatchColor>",self.MatchColor.to_string().as_str())
					.replace("<PHTurns>",self.Turns.to_string().as_str())
					.replace("<PHHisScore>",self.HisScore.to_string().as_str())
					.replace("<PHMyScore>",self.MyScore.to_string().as_str())
					.replace("<PHResult>",self.Result.to_string().as_str())
					.replace("<PHEnLvl>",self.EnLvl.to_string().as_str())
					.replace("<PHMyLvl>",self.MyLvl.to_string().as_str())
					.replace("<PHManas>",self.Manas.to_string().as_str())
			);
		let results = connection.query::<classMatches,String>(mysqlquery);
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

impl classMatches
{
	fn new(&self) -> classMatches
	{
		classMatches
		{
			idMatch: 0,
			idPlayer: 0,
			MatchColor: 0,
			Turns: 0,
			HisScore: 0,
			MyScore: 0,
			Result: 0,
			StartTime: 0,
			EndTime: 0,
			EnLvl: 0,
			MyLvl: 0,
			Manas: 0,
			Duration: 0,
		}
	}


}

