namespace CubaseProjectPlugins
{
    public class PluginDetails
    {
        public string CubaseApplication { get; set; }

        public string CubaseVersion { get; set; }

        public string CubaseReleaseDate { get; set; }

        public string Architecture { get; set; }

        public SortedSet<string> Plugins { get; set; }

        public PluginDetails(
            string cubaseApplication,
            string cubaseVersion,
            string cubaseReleaseDate,
            string architecture,
            SortedSet<string> plugins)
        {
            CubaseApplication = cubaseApplication;
            CubaseVersion = cubaseVersion;
            CubaseReleaseDate = cubaseReleaseDate;
            Architecture = architecture;
            Plugins = plugins;
        }
    }
}
