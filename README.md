# Unit Conversion

A unit converter developed to understand how Rust programs work.

# What’s inside this project?

> This MVP project includes:
> - Length unit conversion (cm, inch, km, miles)
> - Temperature unit conversion (Celsius, Fahrenheit, Kelvin)
> - Conversion history storage
> - List of all available units

# Which Rust technologies are used?

This project showcases several core concepts and libraries in the Rust ecosystem:

- **Module System & Visibility**  
  Folder structure follows crate conventions with `mod`, `pub mod`, and re-export to separate domain from the CLI layer.

  ```rust
  // src/lib.rs
  pub mod domain;
  use crate::domain::units::Unit;
  ```

- **Enum & Derive Macros**  
  Using the `ValueEnum` derive macro from the `clap` crate to map CLI arguments directly to enums.

  ```rust
  #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
  pub enum Unit {
      #[value(alias = "celsius")] Celcius,
      Fahrenheit,
      Kelvin,
      Cm,
      Inch,
      Km,
      Miles,
  }  ```

- **Pattern Matching & Method Dispatch**  
  Unit-to-unit conversion uses pattern matching on unit categories.

  ```rust
  let result = match self.get_category() {
      Category::Temperature => self.convert_temp(to, value),
      Category::Length      => self.convert_length(to, value),
  };
  ```

- **Trait Implementation (`Display`)**  
  Example of implementing a trait for custom formatting.

  ```rust
  impl fmt::Display for Unit {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{:?}", self)
      }
  }
  ```

- **Option Enum (`Some` & `None`)**  
  Rust's `Option<T>` enum is used to represent a value that may or may not be present. `Some(T)` holds a value, while `None` indicates the absence of a value. This helps prevent null pointer exceptions common in other languages.

  In this project, `Option` is used when parsing unit inputs:
  ```rust
  // src/domain/units.rs
  pub fn try_from_input(input: &str) -> Option<Self> {
      Self::value_variants()
          .iter()
          .find(|&&v| v.to_possible_value().unwrap().get_name() == input)
          .copied()
  }

  // src/lib.rs (usage)
  let unit_from = match Unit::try_from_input(from) {
      Some(u) => u,
      None => {
          eprintln!("Error: [ERROR] Satuan asal '{}' tidak dikenali.", from);
          std::process::exit(1);
      }
  };
  ```

- **Closures**  
  Closures are anonymous functions that can capture values from their enclosing scope. They are often used for concise, inline operations.

  Here, a closure is used with the `find` method to search for a matching unit:
  ```rust
  // src/domain/units.rs
  // Inside try_from_input function
  .find(|&&v| v.to_possible_value().unwrap().get_name() == input)
  ```

- **Error Handling & `anyhow`**  
  Error propagation with `Result` and early-return `?`.

  ```rust
  let history = domain::records::load_history()?;
  ```

- **Serialization (Serde)**  
  Persisting history to JSON with `Serialize` & `Deserialize` derives.

  ```rust
  #[derive(Serialize, Deserialize)]
  pub struct ConversionRecord {
      from: String,
      to: String,
      value: f64,
      result: f64,
  }
  ```

- **File I/O**  
  Reading and writing the `conversion.json` file.

  ```rust
  let data = fs::read_to_string(FILE_PATH)?;
  fs::write(FILE_PATH, json_string)?;
  ```

- **Unit Testing (`cargo test`)**  
  Example accuracy tests for conversions.

  ```rust
  #[test]
  fn test_temperature_conversion() {
      let got = Unit::Celcius.convert(&Unit::Kelvin, 0.0).unwrap();
      assert_eq!(got, "0 °C = 273.15 K");
  }
  ```

- **Command-line Interface (CLI)**  
  Sub-commands `convert`, `history`, `list` defined via `clap`.

  ```rust
  #[derive(Subcommand)]
  pub enum Commands {
      Convert { #[arg(long)] from: String, #[arg(long)] to: String, #[arg(long)] value: f64 },
      History,
      List,
  }
  ```

- **Formatting & Linting**  
  Code is formatted with `cargo fmt` and can be linted with `cargo clippy`.

This section illustrates how various Rust features—from its strong type system and ownership-borrowing rules to its crate ecosystem—are leveraged to build a reliable and maintainable CLI application.

# How do ownership and borrowing work in this project?

Rust forces us to think explicitly about data ownership and borrowing. Here are the patterns seen in this project:

1. **Borrowing Parameters vs. Ownership**  
   The `convert` function on `Unit` only *borrows* (`&self`, `&Unit`) because it doesn’t need to take ownership of the enum.

   ```rust
   // src/domain/units.rs
   pub fn convert(&self, to: &Unit, value: f64) -> Result<String, String> {
       // ...
   }
   ```
   By borrowing, we can call `convert` repeatedly on the same `Unit` instance without moving its value.

2. **Taking Ownership When Saving History**  
   When creating a `ConversionRecord`, it is passed to `save_to_history` with *move semantics*—the function needs to store it in a `Vec` and write it to disk.

   ```rust
   let record = ConversionRecord { /* fields */ };
   save_to_history(record)?; // ownership moves into the function
   // record can no longer be used here
   ```

3. **Returning Owned Data**  
   `load_history` returns `Vec<ConversionRecord>`; the `Vec` contains data owned by the caller, so it can be freely manipulated without complex lifetimes.

   ```rust
   pub fn load_history() -> io::Result<Vec<ConversionRecord>> { ... }
   let mut history = load_history()?; // caller now owns the Vec
   history.push(new_record);
   ```

4. **Avoiding Unnecessary Clones**  
   Because `ConversionRecord` is moved, we don’t need to `clone`, which is efficient. If you ever need to keep using the value after saving, you can explicitly call `.clone()`—demonstrating full developer control over memory allocation.

5. **Immutable vs. Mutable Borrow**  
   - In `Unit::convert`, `&self` and `&Unit` are borrowed *immutably*.
   - In `save_to_history`, `load_history()?` yields `mut records`, expressing that we need mutation rights before rewriting the file.

   ```rust
   let mut records = load_history()?; // mutable borrow of local variable
   records.push(record);              // modification
   ```

6. **Lifetime Simplicity**  
   With clear ownership strategies (borrow to read, move to store), we avoid explicit lifetime annotations; the compiler infers them automatically.

In short, this project exemplifies Rust’s core principles:
• Data is read via *borrowing* so references stay valid without duplication.  
• Data is persisted via *ownership transfer*, ensuring no dangling references to moved memory.  
These practices keep memory safe while maintaining performance.

# Acknowledgements

- **Rust Documentation**  
  The official Rust documentation that greatly helped in understanding core concepts and available features.

- **Serde**  
  The serde library that simplifies JSON serialization and deserialization.

- **Clap**  
  The clap library that makes building CLIs effortless.

- **Anyhow**  
  The anyhow library that simplifies error propagation.

- **Cargo**  
  The build tool and package manager that greatly aids Rust development.

- **Dicoding**  
  The Rust learning platform that provides resources and support for Rust developers.

---
Happy coding! 👨‍💻
