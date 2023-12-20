# Alibaba Graph Processor


## Prerequisites

- Python, Pip and Pipenv (virtual environment manager for Python).
- Rust and Cargo.
- Jupyter Notebook.

## How to Run

- Clone the repository.
- In the project directory run `pipenv install`.
- Get into the virtual environment by running `pipenv shell`.
- To build the Rust project run `cargo build`.
- To build and install the executables for Python run `maturin develop`.

## Setting Environment Variables

This project supports three environment variables.

- `FILE_DURATION_IN_SECONDS` - The duration of each Alibaba log file in seconds. This is 180 seconds (3 minutes) and this is fixed by Alibaba. The default value is set to 180. We will not change it.
- `RAW_TRACE_DIR` - The directory where raw trace files downloaded from Alibaba server will get stored. By default this is `<project_root>/data/raw`. But we can override this to any directory inside the env files.
- `WINDOWS_DIR` - The directory where the processed window files will get stored. By default this is `<project_root>/data/windows`. But we can override this to any directory inside the env files.

To update `RAW_TRACE_DIR` and `WINDOWS_DIR` without adding files in git copy the `.env` file to a new file named `.env.local`. Then create entries for `RAW_TRACE_DIR` and / or `WINDOWS_DIR`. The directories in the `env` files have to be absolute directories.

## Command Line Options

## Sample Command

## Library Mode

## Building for Python

## Running Python Notebook