namespace CubaseProjectPlugins;

/// <summary>
/// Plugin specific configuration for the tool.
/// </summary>
public class PluginConfig
{
    /// <summary>
    /// Initializes a new instance of the <see cref="PluginConfig"/> class.
    /// </summary>
    public PluginConfig()
    {
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="PluginConfig"/> class.
    /// </summary>
    /// <param name="guidIgnores">Plugin GUIDs to ignore.</param>
    /// <param name="nameIgnores">Plugin names to ignore.</param>
    public PluginConfig(string[] guidIgnores, string[] nameIgnores)
    {
        GuidIgnores = guidIgnores;
        NameIgnores = nameIgnores;
    }

    /// <summary>
    /// Gets or sets plugin GUIDs which are to be ignored.
    /// </summary>
    [TomlProperty("guid_ignores")]
    public string[] GuidIgnores { get; set; } = Array.Empty<string>();

    /// <summary>
    /// Gets or sets plugin names which are to be ignored.
    /// </summary>
    [TomlProperty("name_ignores")]
    public string[] NameIgnores { get; set; } = Array.Empty<string>();
}

/// <summary>
/// The main configuration structure for the tool.
/// </summary>
public class Config
{
    /// <summary>
    /// Initializes a new instance of the <see cref="Config"/> class.
    /// </summary>
    public Config()
    {
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="Config"/> class.
    /// </summary>
    /// <param name="pathIgnorePatterns">The path patterns to ignore.</param>
    /// <param name="plugins">The plugin related configuration.</param>
    public Config(string[] pathIgnorePatterns, PluginConfig plugins)
    {
        PathIgnorePatterns = pathIgnorePatterns;
        Plugins = plugins;
    }

    /// <summary>
    /// Gets or sets path patterns which determine which projects are skipped.
    /// </summary>
    [TomlProperty("path_ignore_patterns")]
    public string[] PathIgnorePatterns { get; set; } = Array.Empty<string>();

    /// <summary>
    /// Gets or sets configuration related to plugins.
    /// </summary>
    [TomlProperty("plugins")]
    public PluginConfig Plugins { get; set; } = new();
}
