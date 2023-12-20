# Alibaba Graph Processor


## Prerequisites

- Python, Pip and Pipenv (virtual environment manager for Python).
- Rust and Cargo.
- Jupyter Notebook (automatically gets installed as a pipenv dependency).

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

This project can run two commands.

`load` - This command downloads the appropriate trace files from Alibaba server, windows the data and stores the windows in disk.

It supports few options.

  - `start_time` - Start time in seconds to load the trace data (starting from 0).
  - `end_time` - End time in seconds to load the trace data (starting from 0).

If we provide `start_time` and `end_time`, then the breakdown time parameters can be ignored.

  - `start_day` - Start day to load the trace data (0 - 12).
  - `start_hour` - Start hour to load the trace data (0 - 23).
  - `start_minute` - Start minute to load the trace data (0 - 59).
  - `start_second` - Start second to load the trace data (0 - 59).
  - `end_day` - End day to load the trace data (0 - 13, exclusive).
  - `end_hour` - End hour to load the trace data (0 - 24, exclusive).
  - `end_minute` - End minute to load the trace data (0 - 60, exclusive).
  - `start_second` - Start second to load the trace data (0 - 60, exclusive).

Other parameters are,

  - `connection_prop` - Can be either `instance_id` or `microservice_id`. If `instance_id`, then graph edges are connected by individual instance ids, otherwise edges will be connected by microservice ids. Default is `microservice_id`.
  - `window_indexing_type` - Can be `from_zero` or `seq_from_start`. Suppose we are importing from `start_hour=1` and `window_size=60`. If `window_indexing_type` is `from_zero`, then the first window index will be 0. If it is `seq_from_start` then the first window index will be 60. Default is `seq_from_start`.
  - `window_size` `<required>` - The window size in seconds.

`process` - This command processes the produced windows and run some `operation` on them to get some results.

It supports one option.

  - `op` `required` - The operation we want to run on each window. The list is not exhaustive, because we will keep adding more operations. But this should be a valid operation key.

## Sample Command

## Library Mode

## Building for Python

## Running Python Notebook