namespace CubaseProjectPlugins;

public class PluginCounter
{
    public string[] IgnoreNames { get; set; }

    private readonly byte[] _projectBytes;

    private int _index;

    public PluginCounter(byte[] projectBytes, string[] ignoreNames)
    {
        IgnoreNames = ignoreNames;

        _projectBytes = projectBytes;
        _index = 0;
    }

    public PluginDetails GetCounts()
    {
        byte[] pluginUidSearchTerm = Encoding.ASCII.GetBytes("Plugin UID\0");
        byte[] appVersionSearchTerm = Encoding.ASCII.GetBytes("PAppVersion\0");

        // Find every byte that's the letter P.
        SortedSet<string> plugins = new();
        string cubaseApplication = "Cubase";
        string cubaseVersion = "Unknown";
        string cubaseReleaseDate = "Unknown";
        string architecture = "Unknown";

        for (int i = 0; i < _projectBytes.Length; i++)
        {
            // Check if the current byte matches the first byte we're searching for.
            if (_projectBytes[i] != pluginUidSearchTerm[0])
            {
                continue;
            }

            // Check that the next set of bytes relate to a plugin.
            byte[] uidTerm = _projectBytes[i..(i + pluginUidSearchTerm.Length)];
            if (uidTerm.SequenceEqual(pluginUidSearchTerm))
            {
                _index = i + pluginUidSearchTerm.Length + 22;
                string key;
                string name;

                _ = GetToken(); // GUID
                _index += 3;

                key = GetToken();
                if (key != "Plugin Name")
                {
                    continue;
                }

                _index += 5;
                name = GetToken();

                _index += 3;
                key = GetToken();

                if (key == "Original Plugin Name")
                {
                    _index += 5;
                    name = GetToken();
                }

                // Skip names that are to be ignored.
                if (IgnoreNames.Contains(name))
                {
                    continue;
                }

                plugins.Add(name);
                continue;
            }

            // Check that the next set of bytes related to the Cubase version.
            byte[] versionTerm = _projectBytes[i..(i + appVersionSearchTerm.Length)];
            if (versionTerm.SequenceEqual(appVersionSearchTerm))
            {
                _index = i + appVersionSearchTerm.Length + 9;
                cubaseApplication = GetToken();

                _index += 3;
                cubaseVersion = GetToken();

                _index += 3;
                cubaseReleaseDate = GetToken();

                _index += 7;
                try
                {
                    architecture = GetToken();
                }
                catch (IndexOutOfRangeException)
                {
                    // Older 32-bit versions of Cubase didn't list the architecture in the project file.
                    architecture = "WIN32";
                }
            }
        }

        return new PluginDetails(
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
