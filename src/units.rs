pub enum Unit {
    Celcius,
    Fahrenheit,
    Kelvin,
}

impl Unit {
    pub fn convert_to(&self, to: &Unit, temp: f64) -> f64 {
        Self::convert(self, to, temp)
    }

    pub fn convert(from: &Unit, to: &Unit, temp: f64) -> f64 {
        match from {
            Unit::Celcius => match to {
                Unit::Celcius => temp,
                Unit::Fahrenheit => (temp * 1.8) + 32.0,
                Unit::Kelvin => temp + 273.15,
            },
            Unit::Fahrenheit => match to {
                Unit::Celcius => (temp - 32.0) / 1.8,
                Unit::Fahrenheit => temp,
                Unit::Kelvin => ((temp - 32.0) / 1.8) + 273.15,
            },
            Unit::Kelvin => match to {
                Unit::Celcius => temp - 273.15,
                Unit::Fahrenheit => ((temp - 273.15) * 1.8) + 32.0,
                Unit::Kelvin => temp,
            },
        }
    }

    pub fn to_str_glyph(&self) -> &'static str {
        match self {
            Unit::Celcius => "°C",
            Unit::Fahrenheit => "°F",
            Unit::Kelvin => "°K",
        }
    }

    pub fn to_str_long(&self) -> &'static str {
        match self {
            Unit::Celcius => "Celcius",
            Unit::Fahrenheit => "Fahrenheit",
            Unit::Kelvin => "Kelvin",
        }
    }

    pub fn to_str_short(&self) -> &'static str {
        match self {
            Unit::Celcius => "C",
            Unit::Fahrenheit => "F",
            Unit::Kelvin => "K",
        }
    }

    pub fn from_str(s: &str) -> Option<Unit> {
        match s {
            "°C" | "c" | "C" | "celcius" | "Celcius" => Some(Unit::Celcius),
            "°F" | "f" | "F" | "fahrenheit" | "Fahrenheit" => Some(Unit::Fahrenheit),
            "°K" | "k" | "K" | "kelvin" | "Kelvin" => Some(Unit::Kelvin),
            _ => None,
        }
    }
}
