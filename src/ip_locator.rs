use reqwest::blocking::get;
use colored::Colorize;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocationData {
    status: String,
    continent: String,
    country: String,
    country_code: String,
    region_name: String,
    city: String,
    isp: String,
    org: String,
}
impl std::fmt::Display for LocationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if &self.status != "success" {
            return write!(f, "error");
        }
        let mut result = Vec::new();

        if !self.continent.is_empty() {
            result.push(format!("continent: {}", self.continent.bold()));
        }

        if !self.country.is_empty() {
            result.push(format!("country: {} ({})", self.country_code.bold(), self.country.bold()));
        }

        if !self.region_name.is_empty() {
            result.push(format!("region/state: {}", self.region_name.bold()));
        }

        if !self.city.is_empty() {
            result.push(format!("city: {}", self.city.bold()));
        }

        if !self.org.is_empty() {
            result.push(format!("organization: {}", self.org.bold()));
        }

        if !self.isp.is_empty() {
            result.push(format!("isp: {}", self.isp.bold()));
        }

        write!(f, "\t{}", result.join("\n\t"))
    }
}

pub fn locate(ip: &str) -> String {
    let url = format!("http://ip-api.com/json/{ip}?fields=1066523");
    match get(url) {
        Err(e) => {eprintln!("error finding location data for {ip}: {e:?}"); String::new()},
        Ok(content) => {
            // text = content.text().unwrap();
            let data: LocationData = match serde_json::from_reader(content) {
                Ok(s) => s,
                Err(_) => {
                    return format!("invalid response for {}", ip.red().bold()).italic().to_string();
                }
            };

            data.to_string()
        }
    }
}