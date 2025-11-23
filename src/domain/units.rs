use crate::domain::records::{ConversionRecord, save_to_history};
use clap::ValueEnum;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Temperature,
    Length,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Unit {
    // Temperature
    Celsius,
    Fahrenheit,
    Kelvin,
    // Length
    Cm,
    Inch,
    Km,
    Miles,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn round(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor    
}

impl Unit {
    /// Converts a string input to a Unit variant, case-insensitive.
    /// Returns None if the input does not match any unit.
    pub fn try_from_input(input: &str) -> Option<Self> {
        Self::value_variants()
            .iter()
            .find(|&&v| v.to_possible_value().unwrap().get_name() == input)
            .copied()
    }

    /// Returns a vector of all available units
    pub fn get_all_units() -> Vec<Unit> {
        vec![
            Self::Celsius,
            Self::Fahrenheit,
            Self::Kelvin,
            Self::Cm,
            Self::Inch,
            Self::Km,
            Self::Miles,
        ]
    }

    /// Returns a formatted list of all units with numbering and category
    pub fn list_as_string() -> String {
        let units = Self::get_all_units();
        let mut lines = Vec::new();
        lines.push("Satuan yang didukung:".to_string());
        for (idx, unit) in units.iter().enumerate() {
            let category = match unit.get_category() {
                Category::Temperature => "suhu",
                Category::Length => "panjang",
            };
            let name = format!("{:?}", unit).to_lowercase();
            lines.push(format!("{}. [{}] {}", idx + 1, category, name));
        }
        lines.join("\n")
    }

    /// Helper method to get a lowercase display name (correct spelling)
    fn display_name(&self) -> &'static str {
        match self {
            Unit::Celsius => "celsius",
            Unit::Fahrenheit => "fahrenheit",
            Unit::Kelvin => "kelvin",
            Unit::Cm => "cm",
            Unit::Inch => "inch",
            Unit::Km => "km",
            Unit::Miles => "miles",
        }
    }

    fn category_label(cat: &Category) -> &'static str {
        match cat {
            Category::Temperature => "suhu",
            Category::Length => "panjang",
        }
    }

    /// Helper method to get the category of the unit
    pub fn get_category(&self) -> Category {
        match self {
            Self::Celsius | Self::Fahrenheit | Self::Kelvin => Category::Temperature,
            Self::Cm | Self::Inch | Self::Km | Self::Miles => Category::Length,
        }
    }

    /// Convert unit to another unit
    pub fn convert(&self, to: &Unit, value: f64) -> Result<String, String> {
        // Validate Category
        if self.get_category() != to.get_category() {
            return Err(format!(
                "[ERROR] Tidak dapat mengonversi satuan yang berbeda kategori: [{}] {} → [{}] {}",
                Self::category_label(&self.get_category()),
                self.display_name(),
                Self::category_label(&to.get_category()),
                to.display_name()
            ));
        }
        // Convert by category
        let result = match self.get_category() {
            Category::Temperature => round(self.convert_temp(to, value), 0),
            Category::Length => round(self.convert_length(to, value), 4),
        };
        let display_text = format!(
            "{} {} = {} {}",
            value,
            Self::get_symbol(self),
            result,
            Self::get_symbol(to)
        );
        let conversion_record = ConversionRecord {
            from: self.display_name().to_string(),
            to: to.display_name().to_string(),
            value,
            result,
            display_text: display_text.clone(),
        };
        // Save to history
        let _ = save_to_history(conversion_record);

        // Format output with symbol
        Ok(display_text)
    }

    // --- Logic Simbol Output ---
    fn get_symbol(unit: &Unit) -> String {
        match unit {
            Unit::Celsius => "°C".to_string(),
            Unit::Fahrenheit => "°F".to_string(),
            Unit::Kelvin => "K".to_string(),
            Unit::Cm => "cm".to_string(),
            Unit::Inch => "inch".to_string(),
            Unit::Km => "km".to_string(),
            Unit::Miles => "miles".to_string(),
        }
    }

    /// Convert temperature unit (Basis: Celcius)
    fn convert_temp(&self, to: &Unit, val: f64) -> f64 {
        let celsius: f64 = match self {
            Unit::Celsius => val,
            Unit::Fahrenheit => (val - 32.0) * 5.0 / 9.0,
            Unit::Kelvin => val - 273.15,
            _ => val,
        };

        match to {
            Unit::Celsius => celsius,
            Unit::Fahrenheit => (celsius * 9.0 / 5.0) + 32.0,
            Unit::Kelvin => celsius + 273.15,
            _ => celsius,
        }
    }

    /// Convert length unit (Basis: cm)
    fn convert_length(&self, to: &Unit, val: f64) -> f64 {
        let cm = match self {
            Unit::Cm => val,
            Unit::Inch => val * 2.54,
            Unit::Km => val * 100000.0,
            Unit::Miles => val * 160934.4,
            _ => val,
        };
        match to {
            Unit::Cm => cm,
            Unit::Inch => cm / 2.54,
            Unit::Km => cm / 100000.0,
            Unit::Miles => cm / 160934.4,
            _ => cm,
        }
    }
}
