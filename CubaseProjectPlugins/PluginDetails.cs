using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace CubaseProjectPlugins
{
    public class PluginDetails
    {
        public string CubaseVersion { get; set; } = "Unknown";

        public string Architecture { get; set; } = "Unknown";

        public HashSet<string> Plugins { get; set; }

        public PluginDetails(string cubaseVersion, string architecture, HashSet<string> plugins)
        {
            CubaseVersion = cubaseVersion;
            Architecture = architecture;
            Plugins = plugins;
        }
    }
}
