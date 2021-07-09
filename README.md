## vim-profiler 🕒

`vim-profiler` is a wrapper around the `(n)vim --startuptime` command, written in Rust. The binary is called
`vp` and has only been tested on a Unix based system.

### Installation

You can install `vim-profiler` with the rust package manager Cargo:
```bash
$ cargo install vim-profiler
```

### Usage

```
vim-profiler 0.0.1
A vim profiling tool.

USAGE:
    vp [FLAGS] [OPTIONS]

FLAGS:
    -e, --export     Export the results to a CSV file
    -h, --help       Prints help information
    -p, --plot       Plot the data in the terminal
    -r, --reverse    Display the plugin times in reverse order (fastest first)
    -s, --sys        Show system plugins in the output
    -V, --version    Prints version information
    -v, --verbose    Add informative messages during program execution

OPTIONS:
    -c, --command <command>        The command to run, e.g vim or neovim [default: vim]
    -n, --count <count>            The number of plugins to list in the output
    -i, --iter <iter>              The number of iterations
    -x, --precision <precision>    Precision in the output
```

### Prior Art

The API is heavily inspired by the Python script that goes by the same name
[vim-profiler](https://github.com/bchretien/vim-profiler).

A few other notable vim profiling tools include:
- [`hyiltiz/vim-plugins-profile`](https://github.com/hyiltiz/vim-plugins-profile)
- [`dstein64/vim-startuptime`](https://github.com/dstein64/vim-startuptime)
