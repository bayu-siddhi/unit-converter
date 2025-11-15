# **`unitconv`: A Terminal-based Unit Converter**

A simple and fast terminal-based application, built with Rust, for converting temperature and length units. This project serves as an introduction to Rust, covering fundamentals like command-line argument parsing, module organization, error handling, and file I/O.

## **Features**

-   **Temperature Conversion**: Convert between Celsius, Fahrenheit, and Kelvin.
-   **Length Conversion**: Convert between Centimeter, Inch, Kilometer, and Mile.
-   **List Supported Units**: Display a clear list of all available units and their categories.
-   **Conversion History**: Automatically saves every successful conversion to a local file (`data/conversion.json`).
-   **View History**: Display all previous conversions from the command line.

## **Purpose of this Project**

The primary goal of `unitconv` is to be a learning project for getting started with Rust. It demonstrates several key concepts and popular crates in the Rust ecosystem:

-   **CLI Parsing**: Using `clap` to build a robust and user-friendly command-line interface.
-   **Error Handling**: Leveraging the `anyhow` crate for simple and effective error management.
-   **Serialization/Deserialization**: Using `serde` and `serde_json` to save and load conversion history.
-   **Modularity**: Structuring the application into logical modules (`cli`, `converter`, `history`, `units`).
-   **Rust Fundamentals**: Applying core Rust principles like enums, structs, traits, and ownership.

## **Installation**

To get this project up and running on your local machine, you will need to have Rust and Cargo installed. This project was developed and tested using **`Rust 1.90.0`**. If you don't have them, you can install them from the [official Rust website](https://www.rust-lang.org/tools/install).

Follow these steps:

**1. Clone the repository**
Open your terminal and clone this repository to your local machine.

```sh
git clone https://github.com/bayu-siddhi/unit-converter
cd unitconv
```

**2. Build the project**
Use Cargo to build the project. The `--release` flag will compile an optimized executable, which is recommended for general use.

```sh
cargo build --release
```

**3. Run the application**
The executable will be located in the `target/release/` directory. You can run it directly from there.

```sh
./target/release/unitconv --help
```

To make it accessible from anywhere, you can copy the executable inside a directory included in your user `PATH`. Below is an example setup for Linux using `~/.local/bin`:

```sh
mkdir -p ~/.local/bin
cp ./target/release/unitconv ~/.local/bin/
chmod +x ~/.local/bin/unitconv
 
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc
which unitconv
```

After this, you can simply use `unitconv` instead of the full path.

## **Usage**

The application is controlled via subcommands and arguments.

### **View Help**

To see all available commands and options, use the `--help` flag.

```sh
unitconv --help
```

### **Convert Units**

Use the `convert` subcommand with the `--from`, `--to`, and `--value` arguments.

**Examples:**

-   Convert 100 Celsius to Fahrenheit:
    ```sh
    unitconv convert --from celsius --to fahrenheit --value 100
    # Output: 100.0 °C = 212.0 °F
    ```

-   Convert 10 Kilometers to Miles:
    ```sh
    unitconv convert --from km --to miles --value 10
    # Output: 10.0 km = 6.2137 miles
    ```

-   Convert 12 Inches to Centimeters:
    ```sh
    unitconv convert --from inch --to cm --value 12
    # Output: 12.0 inch = 30.48 cm
    ```

### **List Supported Units**

To see a list of all units the application supports, use the `list` command.

```sh
unitconv list
```

**Output:**
```
Supported units:
1. [temperature] celsius
2. [temperature] fahrenheit
3. [temperature] kelvin
4. [length] cm
5. [length] inch
6. [length] km
7. [length] miles
```

### **View Conversion History**

To display a log of all past conversions, use the `history` command.

```sh
unitconv history
```

**Output:**
```
Conversion History:
1. 100.0 °C = 212.0 °F
2. 10.0 km = 6.2137 miles
3. 12.0 inch = 30.48 cm
```
