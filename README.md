# RegSort - File Organizer 🗂️

**RegSort** is a simple and efficient command-line tool designed to help you organize your files based on custom
patterns and rules. It automatically sorts files in your selected folder into predefined directories based on regex
patterns.

### 🚀 Features

- ✅ Automatically move files based on regex patterns defined in a configuration file.
- 🔍 Support for file types, file names, and other pattern-based rules.
- 🧪 Supports "dry-run" mode to simulate the file organization without making changes.
- ⚙️ Customizable configuration to easily define your sorting rules.
- 📝 Log every action taken for traceability and debugging.
- 📂 Create target directories if they don't exist.

---

### 🛠️ How it Works

RegSort watches a specified directory and organizes files into folders based on rules you define in a TOML configuration
file. Each rule consists of:

- A **regex pattern** that matches the file name or type.
- A **target directory** where files matching the pattern will be moved.

For example, you can configure it to move `.pdf` files into a `Documents/Invoices` folder and `.exe` files into a
`Documents/Programs` folder.

---

### 📝 Configuration Example

The configuration file (`config.toml`) is simple and flexible. Here is an example:

```toml
[config]
source_dir = "in"
dry_run = false
log = true

[[rules]]
pattern = ".*\\.txt"
target = "out/txt"

[[rules]]
pattern = ".*\\.csv"
target = "out/csv"
```

The above configuration

- set source dir to `/in`
- does not set `dry-run` mode - changes will be applied
- set log level to `DEBUG`
- moves `.txt` files from `/in/*` to `out/txt` directory
- moves `.csv` files from `/in/*` to `out/csv` directory

---

### 🎬‍ Usage Example

Within this repository run
```bash
export REG_SORT_CONFIG=config.toml
```
And then
```bash
cargo run
```
Add `.txt` file to `/in` repo. It should be moved to `out/txt`.