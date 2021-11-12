namespace CubaseProjectPlugins;

public class PluginCounter
{
    public string[] IgnoreNames { get; set; }

    public PluginCounter(string[] ignoreNames)
    {
        IgnoreNames = ignoreNames;
    }

    public Dictionary<string, int> GetCounts(string path)
    {
        byte[] projectBytes = File.ReadAllBytes(path);

        byte[] pluginUidSearchTerm = Encoding.ASCII.GetBytes("Plugin UID\0");
        byte[] pluginNameSearchTerm = Encoding.ASCII.GetBytes("Plugin Name\0");

        // Find every byte that's the letter P.
        Dictionary<string, int> plugins = new();
        for (int i = 0; i < projectBytes.Length; i++)
        {
            // Check if the current byte matches the first byte we're searching for.
            if (projectBytes[i] != pluginUidSearchTerm[0])
            {
                continue;
            }

            // Check that the next set of bytes match the entire set of bytes we are after.
            byte[] uidTerm = projectBytes[i..(i + pluginUidSearchTerm.Length)];
            if (!uidTerm.SequenceEqual(pluginUidSearchTerm))
            {
                continue;
            }

            int guidLengthIndex = i + pluginUidSearchTerm.Length + 22;
            int guidLength = projectBytes[guidLengthIndex];

            int pluginNameKeyIndex = guidLengthIndex + guidLength + 5;

            byte[] nameTerm = projectBytes[pluginNameKeyIndex..(pluginNameKeyIndex + pluginNameSearchTerm.Length)];
            if (!nameTerm.SequenceEqual(pluginNameSearchTerm))
            {
                continue;
            }

            int pluginNameValueLengthIndex = pluginNameKeyIndex + 17;
            int pluginNameValueLegnth = projectBytes[pluginNameValueLengthIndex];

            int pluginNameValueIndex = pluginNameValueLengthIndex + 1;

            // Grab the plugin name.
            string name = GetNullTerminatedString(
                projectBytes[pluginNameValueIndex..(pluginNameValueIndex + pluginNameValueLegnth)]);

            int extraPropertyLengthIndex = pluginNameValueIndex + pluginNameValueLegnth + 3;
            int extraPropertyLength = projectBytes[extraPropertyLengthIndex];

            int extraPropertyIndex = extraPropertyLengthIndex + 1;

            string extraProperty = GetNullTerminatedString(
                projectBytes[extraPropertyIndex..(extraPropertyIndex + extraPropertyLength)]);

            if (extraProperty == "Original Plugin Name")
            {
                int originalPluginNameLengthIndex = extraPropertyIndex + extraPropertyLength + 5;
                int originalPluginNameLength = projectBytes[originalPluginNameLengthIndex];
                int originalPluginNameIndex = originalPluginNameLengthIndex + 1;

                string originalPluginName = GetNullTerminatedString(
                    projectBytes[originalPluginNameIndex..(originalPluginNameIndex + originalPluginNameLength)]
                );

                name = originalPluginName;
            }

            // Skip names that are to be ignored.
            if (IgnoreNames.Contains(name))
            {
                continue;
            }

            if (plugins.ContainsKey(name))
            {
                plugins[name]++;
            }
            else
            {
                plugins[name] = 1;
            }
        }

        return plugins;
    }

    private string GetNullTerminatedString(byte[] buffer)
    {
        StringBuilder sb = new();
        for (int i = 0; i < buffer.Length && buffer[i] != 0; i++)
        {
            sb.Append((char)buffer[i]);
        }
        return sb.ToString();
    }

    private string[] GetNullTerminatedStrings(byte[] buffer, int startIndex, int count)
    {
        List<string> strings = new();
        int i = startIndex;

        while (count-- > 0)
        {
            StringBuilder sb = new();
            for (; buffer[i] != 0; i++)
            {
                sb.Append((char)buffer[i]);
            }
            strings.Add(sb.ToString());

            while (buffer[i] == 0)
            {
                i++;
            }
        }

        return strings.ToArray();
    }
}
