namespace CubaseProjectPlugins;

/// <summary>
/// Represents a plugin within a Cubase project.
/// </summary>
public class Plugin : IComparable<Plugin>, IEquatable<Plugin>
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

        if (Guid == other.Guid)
        {
            return 0;
        }

        return Name.CompareTo(other.Name);
    }

    public override int GetHashCode()
    {
        return Guid.GetHashCode();
    }

    /// <summary>
    /// Compares the current plugin with another.
    /// </summary>
    /// <param name="other">The other plugin object to compare against.</param>
    /// <returns>A boolean indicating whether both objects are equal.</returns>
    public bool Equals(Plugin? other)
    {
        if (other is null)
        {
            return false;
        }

        // Optimisation for a common success case.
        if (ReferenceEquals(this, other))
        {
            return true;
        }

        // If run-time types are not exactly the same, return false.
        if (this.GetType() != other.GetType())
        {
            return false;
        }

        // Return true if the fields match.
        return Guid == other.Guid && Name == other.Name;
    }
}
