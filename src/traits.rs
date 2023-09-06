#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use mysql::PooledConn;

#[allow(non_camel_case_types)]
pub trait UEPObject
{
	// GETERS

	// commun à tous
	fn getID(&self) -> u32;
	// commun à Comments, Players
	fn getString(&self) -> String;

	// spécifique à Comments
	fn getMatchID(&self) -> u32;

	// commun à Players et Matches
	fn getColor(&self) -> u8;
	fn getScoreP(&self) -> u128;
	fn getScoreO(&self) -> u128;

	// spécifique à Players
	fn getVictories(&self) -> u32;
	fn getConcedes(&self) -> u32;
	fn getDefeats(&self) -> u32;
	fn getDraws(&self) -> u32;

	fn getMatches(&self) -> u32;

	// spécifique à Matches
	fn getPlayerID(&self) -> u32;
	fn getTurns(&self) -> u32;
	fn getResult(&self) -> u8;
	fn getStartTime(&self) -> u64;
	fn getEndTime(&self) -> u64;
	fn getManasAmount(&self) -> u8;
	fn getMatchDuration(&self) -> u64;

	// commun à Matches et Mythics
	fn getLvlP(&self) -> u32;
	fn getLvlO(&self) -> u32;

	fn getFromDB(&mut self, connection :&mut PooledConn, id :u32) -> Self;
	fn updateme(self,connection :&mut PooledConn) -> bool;
	fn insertme(self,connection :&mut PooledConn) -> u32;
}




