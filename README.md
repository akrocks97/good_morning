## Good_morning
This is a rust binary crate, which fetches the current weather in at given Longitude/Latitude.
Additionally it also, adds a path to the release executable in the bashrc, 
so that you can get the weather on every terminal boot.
This works for Unix systems (Mac OS). Should work for linux systems, will need some modification though.

## Usage
You need rustup and cargo set up in your system
You may want to sent the following variables in you bashrc file.
Typically it resides in ```$HOME/.bashrc```.
You need an API access key from https://openweathermap.org/ (You can get it by creating a free account and navigation to https://home.openweathermap.org/api_keys)

| Environment Variable | Description |
| -----------------------------     | ---------------- |
| OPEN_WEATHER_API_KEY |  API access key  |
| OPEN_WEATHER_LAT         |  Latitude for you desired location update |
| OPEN_WEATHER_LON        | Longitude for you desired location update |
| BASHRC_PATH                    | Path for your bashrc file, will default to ```$HOME/.bashrc``` if not set |

##Build command
```
git clone git@github.com:akrocks97/good_morning.git
cd good_morning
cargo build --release
cargo run --release
```