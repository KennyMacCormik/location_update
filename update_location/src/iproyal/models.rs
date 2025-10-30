use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Root {
    pub prefix: String,
    pub countries: Vec<Country>,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub ip_availability: Option<String>,
    #[serde(default)]
    pub cities: Option<Container<City>>,
    #[serde(default)]
    pub states: Option<Container<State>>,
}

#[derive(Debug, Deserialize)]
pub struct State {
    pub code: String,
    pub name: String,
    pub ip_availability: Option<String>,
    #[serde(default)]
    pub cities: Option<Container<City>>,
    #[serde(default)]
    pub isps: Option<Container<Isp>>,
}

#[derive(Debug, Deserialize)]
pub struct City {
    pub code: String,
    pub name: String,
    pub ip_availability: Option<String>,
    #[serde(default)]
    pub isps: Option<Container<Isp>>,
}

#[derive(Debug, Deserialize)]
pub struct Isp {
    pub code: String,
    pub name: String,
    pub ip_availability: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Container<T> {
    pub prefix: String,
    pub options: Vec<T>,
}