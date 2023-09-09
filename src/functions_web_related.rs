#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use actix_web::{get, HttpResponse};

#[get("/")]
async fn get_index() -> HttpResponse
{
	let mut htmldata = feedme("./www/html/index.html");
	let cssdata = feedme ("./www/css/index.css");

	htmldata = htmldata.replace("{{BASE_CSS}}",&cssdata);

	HttpResponse::Ok().content_type("text/html").body(htmldata)
}

pub fn feedme(file_path: &str) -> String
{
	let datas = fs::read_to_string(file_path);
	if datas.is_err()
	{
		error!("{}",datas.as_ref().unwrap_err());
		String::from(datas.unwrap_err().to_string())
	}
	else
	{
		datas.unwrap()
	}
}
