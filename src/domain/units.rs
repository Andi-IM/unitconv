use clap::ValueEnum;

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Temperature,
    Length,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Unit {
    // Temperature
    Celcius,
    Fahrenheit,
    Kelvin,
    // Length
    Cm,
    Inch,
    Km,
    Miles,
}

impl Unit {
    /// Helper method to get the category of the unit
    pub fn get_category(&self) -> Category {
        match self {
            Self::Celcius | Self::Fahrenheit | Self::Kelvin => Category::Temperature,
            Self::Cm | Self::Inch | Self::Km | Self::Miles => Category::Length,
        }
    }

    /// Convert unit to another unit
    pub fn convert(&self, to: &Unit, value: f64) -> Result<String, String> {
        // Validate Category
        if self.get_category() != to.get_category() {
            return Err(format!(
                "Unit category mismatch: {:?} != {:?}",
                self.get_category(),
                to.get_category()
            ));
        }
        // Convert by category
        let result = match self.get_category() {
            Category::Temperature => self.convert_temp(to, value),
            Category::Length => self.convert_length(to, value),
        };
        // Format output with symbol
        Ok(format!(
            "{} {} = {} {}",
            value,
            Self::get_symbol(self),
            result,
            Self::get_symbol(to)
        ))
    }

    // --- Logic Simbol Output ---
    fn get_symbol(unit: &Unit) -> String {
        match unit {
            Unit::Celcius => "°C".to_string(),
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
        let celcius = match self {
            Unit::Celcius => val,
            Unit::Fahrenheit => (val - 32.0) * 5.0 / 9.0,
            Unit::Kelvin => val - 273.15,
            _ => val,
        };

        match to {
            Unit::Celcius => celcius,
            Unit::Fahrenheit => (celcius * 9.0 / 5.0) + 32.0,
            Unit::Kelvin => celcius + 273.15,
            _ => celcius,
        }
    }

    /// Convert length unit (Basis: cm)
    fn convert_length(&self, to: &Unit, val: f64) -> f64 {
        let cm = match self {
            Unit::Cm => val,
            Unit::Inch => val * 2.54,
            Unit::Km => val * 1000.0,
            Unit::Miles => val * 1609.34,
            _ => val,
        };
        match to {
            Unit::Cm => cm,
            Unit::Inch => cm / 2.54,
            Unit::Km => cm / 1000.0,
            Unit::Miles => cm / 1609.34,
            _ => cm,
        }
    }
}
