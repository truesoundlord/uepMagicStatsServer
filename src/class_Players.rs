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
pub struct classPlayers
{
	idPlayer :u32,
	Alias :String,
	Colors :u8,
	Victories :u32,
	Conceded :u32,
	Defeated :u32,
	Draws :u32,
	MyScore :u128,
	HisScore :u128,
	MatchesDone :u32,
}

impl FromRow for classPlayers
{
	fn from_row_opt(row: Row) -> Result<Self, FromRowError> where Self: Sized
	{
		let tmp :classPlayers = classPlayers
		{
			idPlayer: row.get::<u32, usize>(0).unwrap(),
			Alias: row.get::<String, usize>(1).unwrap(),
			Colors: row.get::<u8, usize>(2).unwrap(),
			Victories: row.get::<u32, usize>(3).unwrap(),
			Conceded: row.get::<u32, usize>(4).unwrap(),
			Defeated: row.get::<u32, usize>(5).unwrap(),
			Draws: row.get::<u32, usize>(6).unwrap(),
			MyScore: row.get::<u128, usize>(7).unwrap(),
			HisScore: row.get::<u128, usize>(8).unwrap(),
			MatchesDone: row.get::<u32, usize>(9).unwrap(),
		};
		Ok(tmp)
	}
}

impl traits::UEPObject for classPlayers
{
	fn getID(&self) -> u32
	{
		self.idPlayer
	}

	fn getString(&self) -> String
	{
		self.Alias.clone()
	}

	fn getMatchID(&self) -> u32
	{
		0
	}

	fn getColor(&self) -> u8
	{
		self.Colors
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
		self.Victories
	}

	fn getConcedes(&self) -> u32
	{
		self.Conceded
	}

	fn getDefeats(&self) -> u32
	{
		self.Defeated
	}

	fn getDraws(&self) -> u32
	{
		self.Draws
	}

	fn getMatches(&self) -> u32
	{
		self.MatchesDone
	}

	fn getPlayerID(&self) -> u32
	{
		self.idPlayer
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
		let mysqlquery = String::from("SELECT * FROM Players WHERE idMatch = <PHid>").replace("<PHid>",id.to_string().as_str());
		let results = connection.query::<classPlayers,String>(mysqlquery);
		if results.is_ok()
		{
			*self=classPlayers::from(results.unwrap().get(0).unwrap().clone());
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
				"UPDATE Players SET Alias = '<PHAlias>',
						SET Colors = <PHColor>,
						Victories = <PHVictories>,
						Conceded = <PHConceded>,
						Defeated = <PHDefeated>,
						Draws = <PHDraws>,
						MyScore = <PHMyScore>,
						HisScore = <PHHisScore>,
						MatchesDones = <PHMatchesDone>,
						WHERE idPlayer=<PHid>"
					.replace("<PHAlias>",&self.Alias.clone())
					.replace("<PHColor>",self.Colors.to_string().as_str())
					.replace("<PHVictories>",self.Victories.to_string().as_str())
					.replace("<PHConceded>",self.Conceded.to_string().as_str())
					.replace("<PHDefeated>",self.Defeated.to_string().as_str())
					.replace("<PHDraws>",self.Draws.to_string().as_str())
					.replace("<PHMyScore>",self.MyScore.to_string().as_str())
					.replace("<PHHisScore>",self.HisScore.to_string().as_str())
					.replace("<PHMatchesDone>",self.MatchesDone.to_string().as_str())
					.replace("\'","\\'")
			);
		let results = connection.query::<classPlayers,String>(mysqlquery);
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
				"INSERT INTO Players (Alias,Colors,Victories,Conceded,Defeated,Draws,MyScore,HisScore,MatchesDones)
						VALUES ('<PHAlias>',<PHColor>,<PHVictories>,<PHConceded>,<PHDefeated>,<PHDraws>,<PHMyScore>,<PHHisScore>,<PHMatchesDone>)"
					.replace("<PHAlias>",&self.Alias.clone())
					.replace("<PHColor>",self.Colors.to_string().as_str())
					.replace("<PHVictories>",self.Victories.to_string().as_str())
					.replace("<PHConceded>",self.Conceded.to_string().as_str())
					.replace("<PHDefeated>",self.Defeated.to_string().as_str())
					.replace("<PHDraws>",self.Draws.to_string().as_str())
					.replace("<PHMyScore>",self.MyScore.to_string().as_str())
					.replace("<PHHisScore>",self.HisScore.to_string().as_str())
					.replace("<PHMatchesDone>",self.MatchesDone.to_string().as_str())
					.replace("\'","\\'")
			);
		let results = connection.query::<classPlayers,String>(mysqlquery);
		if results.is_err()
		{
			eprintln!("[{}] -> {}","ERROR".red().bold().blink(),results.as_ref().unwrap_err().to_string());
			0
		}
		else
		{
			mysqlquery = String::from("SELECT MAX(idPlayer) FROM Players");
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

impl classPlayers
{
	fn new(&self) -> classPlayers
	{
		classPlayers
		{
			idPlayer: 0,
			Alias: "".to_string(),
			Colors: 0,
			Victories: 0,
			Conceded: 0,
			Defeated: 0,
			Draws: 0,
			MyScore: 0,
			HisScore: 0,
			MatchesDone: 0,
		}
	}
}