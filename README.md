# Cubase Project

This program will analyse a directory containing Cubase projects recursively and show the version
of Cubase the project was created with along with a list all the plugins used (with their GUIDs).

## Limitations

The tool works for projects created in Cubase 4 or later.  Older projects (e.g. those created on
SX3) will only show the Cubase version but no plugins will be listed and architecture may be
assumed to be 32-bit.

## Configuring the Tool

You may optionally create a TOML config file for the utility which includes certain paths and
plugins to exclude from output. The default location where the tool will search for the config
file is expected is **~/.config/cubase-project-plugins.toml**.

The config file should look similar to that below:

```toml
# The path patterns to ignore.
path_ignore_patterns = [
    "**/Ignore Path 1/*.cpr",
    "**/Ignore Path 2/*.cpr",
]

[projects]
# Specify which project architectures will be included in the output.
report_32_bit = true
report_64_bit = true

[plugins]
# Plugin GUIDs to ignore and exclude from output.
guid_ignores = [
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
]

# Plugin names to ignore and exclude from output.
name_ignores = [
    "Plugin1",
    "Plugin2",
]
```

You may see the sample config **config.sample.toml** for inspiration.

## Running the Tool

### Usage

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install the tool

    ```
    cargo install --git https://github.com/fgimian/cubase-project-plugins.git
    ```

3. You may now run the tool using the `cubase-project-plugins` executable

Please use the `-h/--help` option for further usage instructions.

## License

Cubase Project Plugins is released under the **MIT** license. Please see the
[LICENSE](https://github.com/fgimian/cubase-project-plugins/blob/main/LICENSE) file for more
details.
