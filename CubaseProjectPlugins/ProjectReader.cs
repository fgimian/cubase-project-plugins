namespace CubaseProjectPlugins;

/// <summary>
/// Determines the used plugins in a Cubase project along with related version of Cubase which the
/// project was created on by parsing the binary in a *.cpr file.
/// </summary>
public class ProjectReader
{
    private readonly byte[] _projectBytes;

    private int _index;

    /// <summary>
    /// Initializes a new instance of the <see cref="ProjectReader"/> class.
    /// </summary>
    /// <param name="projectBytes">The binary bytes from a *.cpr Cubase project file.</param>
    /// <param name="guidIgnores">All plugin GUIDs which should be ignored.</param>
    /// <param name="nameIgnores">All plugin names which should be ignored.</param>
    public ProjectReader(
        byte[] projectBytes, string[]? guidIgnores = null, string[]? nameIgnores = null)
    {
        GuidIgnores = guidIgnores ?? Array.Empty<string>();
        NameIgnores = nameIgnores ?? Array.Empty<string>();

        _projectBytes = projectBytes;
        _index = 0;
    }

    /// <summary>
    /// Gets or sets all plugin GUIDs that should not captured.  Typically this will be the plugins
    /// which are included in Cubase itself.  This is a more accurate way of excluding plugins
    /// than using their name.
    /// </summary>
    public string[] GuidIgnores { get; set; }

    /// <summary>
    /// Gets or sets all plugin names that should not captured.  Typically this will be the plugins
    /// which are included in Cubase itself.
    /// </summary>
    public string[] NameIgnores { get; set; }

    /// <summary>
    /// Obtains all project details including Cubase version and plugins used.
    /// </summary>
    /// <returns>An instance of <see cref="ProjectDetails"/>containing project details.</returns>
    public ProjectDetails GetProjectDetails()
    {
        byte[] pluginUidSearchTerm = Encoding.ASCII.GetBytes("Plugin UID\0");
        byte[] appVersionSearchTerm = Encoding.ASCII.GetBytes("PAppVersion\0");

        // Find every byte that's the letter P.
        SortedSet<Plugin> plugins = new();
        string cubaseApplication = "Cubase";
        string cubaseVersion = "Unknown";
        string cubaseReleaseDate = "Unknown";
        string architecture = "Unknown";

        for (int i = 0; i < _projectBytes.Length; i++)
        {
            // Check if the current byte matches the first byte we're searching for.
            if (_projectBytes[i] != (char)'P')
            {
                continue;
            }

            // Check that the next set of bytes related to the Cubase version.
            try
            {
                byte[] versionTerm = _projectBytes[i..(i + appVersionSearchTerm.Length)];
                if (versionTerm.SequenceEqual(appVersionSearchTerm))
                {
                    _index = i + appVersionSearchTerm.Length + 9;
                    try
                    {
                        cubaseApplication = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        throw new InvalidDataException("Unable to obtain the app name");
                    }

                    _index += 3;
                    try
                    {
                        cubaseVersion = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        throw new InvalidDataException("Unable to obtain app version");
                    }

                    _index += 3;
                    try
                    {
                        cubaseReleaseDate = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        throw new InvalidDataException("Unable to obtain the app release date");
                    }

                    _index += 7;
                    try
                    {
                        architecture = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        // Older 32-bit versions of Cubase didn't list the architecture in the project file.
                        architecture = "Not Specified";
                    }
                }
            }
            catch (ArgumentOutOfRangeException)
            {
                // Ignore situations where we've gone beyond the end of the file.
            }

            // Check that the next set of bytes relate to a plugin.
            try
            {
                byte[] uidTerm = _projectBytes[i..(i + pluginUidSearchTerm.Length)];
                if (uidTerm.SequenceEqual(pluginUidSearchTerm))
                {
                    _index = i + pluginUidSearchTerm.Length + 22;

                    string key;
                    string guid;
                    string name;

                    try
                    {
                        guid = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        throw new InvalidDataException("Unable to obtain the plugin GUID");
                    }

                    _index += 3;
                    try
                    {
                        key = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        throw new InvalidDataException("Unable to obtain a required key token");
                    }

                    if (key != "Plugin Name")
                    {
                        continue;
                    }

                    _index += 5;
                    try
                    {
                        name = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        throw new InvalidDataException("Unable to obtain the plugin name");
                    }

                    _index += 3;
                    try
                    {
                        key = GetToken();
                    }
                    catch (IndexOutOfRangeException)
                    {
                        throw new InvalidDataException("Unable to obtain a required key token");
                    }

                    if (key == "Original Plugin Name")
                    {
                        _index += 5;
                        try
                        {
                            name = GetToken();
                        }
                        catch (IndexOutOfRangeException)
                        {
                            throw new InvalidDataException(
                                "Unable to obtain the original plugin name");
                        }
                    }

                    // Skip GUIDs that are to be ignored.
                    if (GuidIgnores.Contains(guid))
                    {
                        continue;
                    }

                    // Skip names that are to be ignored.
                    if (NameIgnores.Contains(name))
                    {
                        continue;
                    }

                    plugins.Add(new Plugin(guid, name));
                    continue;
                }
            }
            catch (ArgumentOutOfRangeException)
            {
                // Ignore situations where we've gone beyond the end of the file.
            }
        }

        return new ProjectDetails(
            cubaseApplication, cubaseVersion, cubaseReleaseDate, architecture, plugins);
    }

    private string GetToken()
    {
        int length = _projectBytes[_index];
        _index++;

        byte[] buffer = _projectBytes[_index..(_index + length)];
        StringBuilder sb = new();
        for (int i = 0; i < buffer.Length && buffer[i] != 0; i++)
        {
            sb.Append((char)buffer[i]);
        }

        string text = sb.ToString();
        _index += length;
        return text;
    }
}
