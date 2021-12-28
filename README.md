# Cubase Project Plugins

This project will analyse a directory containing Cubase projects recursively and show the version
of Cubase the project was created with along with a list all the plugins used (with their GUIDs).

## Limitations

The tool works for projects created in Cubase 4 or later.  Older projects (e.g. those created on
SX3) will only show the Cubase version but no plugins will be listed and architecture may be
assumed to be 32-bit.

## Configuring the Tool

You may optionally create a TOML config file for the utility which includes certain paths and
plugins to exclude from output.

The config file should look similar to that below:

```toml
# The path patterns to ignore.
path_ignore_patterns = [
    "*/Ignore Path 1/*",
    "*/Ignore Path 2/*",
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

1. Install the [.NET 6.0 SDK](https://dotnet.microsoft.com/download).
2. Open a PowerShell prompt and build the project

    ```powershell
    cd .\cubase-project-plugins\CubaseProjectPlugins
    dotnet run -c Release -- --path H:\Production --config-path config.toml
    ```

You may optionally redirect the output to a file using the `>` operator.

## License

Cubase Project Plugins is released under the **MIT** license. Please see the
[LICENSE](https://github.com/fgimian/cubase-project-plugins/blob/main/LICENSE) file for more
details.
