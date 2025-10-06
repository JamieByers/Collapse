# Collapse

## What is Collapse?

This is a simple rust CLI tool I created due to my cluttered file system. I find myself creating many folders within folders, and zip files automatically extracting into their own folder. To help declutter my messy files I created this program to collapse folders by taking all of the files from the folder and moving them into the parent folder.

## Installation

There is currently no installer. However you can download the binary from the target folder or clone the repository and build it yourself.

I currently have the executable located in my "/Users/jamiebyers/.local/bin/collapse" (MacOS)
This was then paired with an alias in my .zshrc file:

```bash
alias collapse="/Users/jamiebyers/.local/bin/collapse"
```

## Usage

The CLI can be run with the following syntax:

```bash
collapse <folder_name> [--debug] [--help]
```

### Arguments
- `<folder_name>`: The name of the folder you want to collapse. This argument is required.
- `--debug`: Optional flag to enable debug mode, which provides more detailed output during execution.
- `--help`: Optional flag to display help information about the CLI usage.

### Example
To collapse a folder named "example_folder", you would run:

```bash
# Basic usage
collapse my_folder

# With debugging information
collapse my_folder --debug

# View usage help
collapse --help
```

