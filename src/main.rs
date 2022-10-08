use reqwest;
use std::error::Error;
extern crate tokio;
use json;
use std::env;
use std::format;
use colored::*;

struct WeatherIcon {
    code: String,
    icon: String,
}

#[derive(Debug)]
struct WeatherInfo {
    weather_string: String,
    icon: String,
    temperature: String,
    feels_like: String,
    location: String,
}

fn print_weather(info: &WeatherInfo) {
    println!("Location: {}", info.location.green());
    println!("Status: {} {}", info.weather_string.green(), info.icon.green());
    println!("Temp: {}", info.temperature.green());
    println!("Feels Like: {}", info.feels_like.green());

}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let icons: Vec<WeatherIcon> = vec![
        WeatherIcon {code: String::from("01d"),icon: String::from(""), },
        WeatherIcon {code: String::from("01n"),icon: String::from(""), },
        WeatherIcon {code: String::from("02d"),icon: String::from(""), },
        WeatherIcon {code: String::from("02n"),icon: String::from(""), },
        WeatherIcon {code: String::from("03"), icon: String::from(""), },
        WeatherIcon {code: String::from("04"), icon: String::from(""), },
        WeatherIcon {code: String::from("09"), icon: String::from(""), },
        WeatherIcon {code: String::from("10n"),icon: String::from(""), },
        WeatherIcon {code: String::from("10d"),icon: String::from(""), },
        WeatherIcon {code: String::from("11"), icon: String::from(""), },
        WeatherIcon {code: String::from("13"), icon: String::from("ﰕ"), },
        WeatherIcon {code: String::from("50"), icon: String::from(""), },
    ];
    let api_key = env::var("OPEN_WEATHER_API_KEY").expect("Error getting OPEN_WEATHER_API_KEY");
    let lat = env::var("OPEN_WEATHER_LAT").expect("Error getting OPEN_WEATHER_LAT");
    let lon = env::var("OPEN_WEATHER_LON").expect("Error getting OPEN_WEATHER_LON");

    let http_req = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}",
        lat, lon, api_key
    );

    let response = reqwest::get(http_req).await?.text().await?;

    let json_value = json::parse(response.as_str()).unwrap();
    let icon = match json_value["weather"][0]["icon"] {
        json::JsonValue::Short(icon) => icon.as_str().to_owned(),
        _ => String::from("Err"),
    };
    

    let mut print_icon = String::new();
    for weather_icons in icons {
        if icon.contains(weather_icons.code.as_str()) {
            print_icon = print_icon + &weather_icons.icon;
        }
    };
    let temp = &json_value["weather"][0];
    let current_weather = WeatherInfo {
        weather_string: String::from(format!("{}, {}", temp["main"], temp["description"])),
        icon: print_icon.clone(),
        temperature: String::from(format!("{} ",json_value["main"]["temp"].as_f64().expect("No Valid temperature"))),
        feels_like: String::from(format!("{}  ",json_value["main"]["feels_like"].as_f64().expect("No valid temperature"))),
        location: String::from(json_value["name"].as_str().expect("No Name"))
    };
    print_weather(&current_weather);

    Ok(())
}
