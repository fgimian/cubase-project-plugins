namespace CubaseProjectPlugins;

/// <summary>
/// Represents a plugin within a Cubase project.
/// </summary>
public class Plugin : IComparable<Plugin>
{
    /// <summary>
    /// Initializes a new instance of the <see cref="Plugin"/> class.
    /// </summary>
    /// <param name="guid">The globally unique identifier for the plugin.</param>
    /// <param name="name">The name of the plugin.</param>
    public Plugin(string guid, string name)
    {
        Guid = guid;
        Name = name;
    }

    /// <summary>
    /// Gets or sets the globally unique identifier for the plugin.
    /// </summary>
    public string Guid { get; set; }

    /// <summary>
    /// Gets or sets the name of the plugin.
    /// </summary>
    public string Name { get; set; }

    /// <summary>
    /// Compares the current plugin to another one provided.
    /// </summary>
    /// <param name="other">Another plugin to compare against.</param>
    /// <returns>A integer indicating whether the current plugin is the same as the other.</returns>
    public int CompareTo(Plugin? other)
    {
        if (other == null)
        {
            return 1;
        }

        return Guid.CompareTo(other.Guid);
    }
}
