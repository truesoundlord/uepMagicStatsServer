#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use actix_web::{HttpResponse, web};

use crate::class_Matches::classMatches;

pub fn get_index () -> HttpResponse
{
	HttpResponse::Ok().content_type("text/html").body
	(
		r#"
				<HTML>
				<HEAD>
				<TITLE>Rust server</TITLE>
				</HEAD>
				<BODY>
				</BODY>
				</HTML>
		"#
	)
}

pub fn actions(form :web::Form<classMatches>)
{

}
