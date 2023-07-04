use std::fs::read_to_string;
use clap::Parser;
use reqwest::header::USER_AGENT;
extern crate colored;
use colored::*;


const CONFIG_FILE: &str = "/home/princess/.config/rweather.conf";

#[derive(Parser, Debug)]
#[command(name = "RWeather")]
#[command(author = "Chloe <vincent@gfidi.com>")]
#[command(version = "1.0")]
#[command(about = "Basic CLI Weather Forecasting using the weather.gov API", long_about = None)]
struct Args {
   //How many days out do you want your report to go? Max: 7
   #[arg(short = 'f', long, default_value_t = 2)]
   forecast_length: usize,
}

fn _first_time_setup() { //W.I.P
    println!("Welcome to RWeather. We need your approximate co-ordinates to send you your weather report!");
}

fn print_forecast(json: serde_json::Value, mut days: usize, area_name: String, state_name: String) {
   let mut i: usize = 0;
   if days > 7 {
		days = 7;
   }
   if days > 1 {
		days *= 2;
		days -= 1;
   }
   println!("Forecast for {} {}", area_name.bold(), state_name.bold());
   while i <= days {
		println!("Weather for {}", json["properties"]["periods"][i]["name"].to_string().replace("\"", "").red().bold().underline());
		let temperature: String = json["properties"]["periods"][i]["temperature"].to_string().replace("\"", "");
		let temperature_final: String = temperature.to_owned() + &"F".to_string();
		if temperature.parse::<i32>().unwrap() <= 32 {
			println!("Temperature: {}", temperature_final.blue());
		} else if temperature.parse::<i32>().unwrap() >= 85 {
			println!("Temperature: {}", temperature_final.red());
		} else if temperature.parse::<i32>().unwrap() <= -18 {
			println!("Temperatre: {}", temperature_final.cyan());
		} else {
			println!("Temperature: {}", temperature_final.green());
		}
		let wind_speed: String = json["properties"]["periods"][i]["windSpeed"].to_string().replace("\"", "");
		let wind_dir: String = json["properties"]["periods"][i]["windDirection"].to_string().replace("\"", "");
		let mut test_wind_speed: String = wind_speed.clone();
		let cut_offset = test_wind_speed.find(" ").unwrap_or(test_wind_speed.len());
		test_wind_speed.replace_range(cut_offset.., "");
		if test_wind_speed.parse::<i32>().unwrap() <= 32 {
			println!("Wind: {} from the {}", wind_speed.green(), wind_dir.underline());
		} else if test_wind_speed.parse::<i32>().unwrap() <= 64 {
			println!("Wind: {} from the {}", wind_speed.yellow(), wind_dir.underline());
		} else if test_wind_speed.parse::<i32>().unwrap() >= 75 {
			println!("Wind: {} from the {}", wind_speed.purple(), wind_dir.underline());
		}
		println!("Forecast: {}\n", json["properties"]["periods"][i]["shortForecast"].to_string().replace("\"", "").underline());
		i+=1;
   }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let args: Args = Args::parse(); 
	let coords: String = read_to_string(CONFIG_FILE).unwrap().to_owned();
	let mut weather_gov_site: String = "https://api.weather.gov/points/".to_owned();
	weather_gov_site.push_str(&coords);
	//Request 1: Co-ordinates -> URL to fetch weather
	let points_get: String = reqwest::Client::new()
		.get(weather_gov_site)
		.header(USER_AGENT, "RWeather v1.0, vincent@gfidi.com")
		.send()
		.await?
		.text()
		.await?;
	let points_json: serde_json::Value = serde_json::from_str(points_get.as_str()).unwrap();
	let forecast_url: String = points_json["properties"]["forecast"].to_string().replace("\"", "");
	//Request 2: Fetch Weather from weather.gov
	let forecast_get: String = reqwest::Client::new()
		.get(forecast_url)
		.header(USER_AGENT, "RWeather v1.0, vincent@gfidi.com")
		.send()
		.await?
		.text()
		.await?;
	let forecast_json: serde_json::Value = serde_json::from_str(forecast_get.as_str()).unwrap();
	print_forecast(forecast_json, args.forecast_length, points_json["properties"]["relativeLocation"]["properties"]["city"].to_string().replace("\"", ""), points_json["properties"]["relativeLocation"]["properties"]["state"].to_string().replace("\"", ""));
	Ok(())
	//Later: Add checking if file does not exist
   
}