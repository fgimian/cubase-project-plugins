namespace CubaseProjectPlugins
{
    /// <summary>
    /// Captures the Cubase version and all plugins used for a Cubase project.
    /// </summary>
    public class ProjectDetails
    {
        /// <summary>
        /// The Cubase application name.
        /// </summary>
        public string CubaseApplication { get; set; }

        /// <summary>
        /// The Cubase version used to create the project.
        /// </summary>
        public string CubaseVersion { get; set; }

        /// <summary>
        /// The release date of the Cubase version used.
        /// </summary>
        public string CubaseReleaseDate { get; set; }

        /// <summary>
        /// The system architecture of the Cubase version used.
        /// </summary>
        public string Architecture { get; set; }

        /// <summary>
        /// All plugin names used in the project.
        /// </summary>
        public SortedSet<string> Plugins { get; set; }

        /// <summary>
        /// Initialises a new instance of the <see cref="ProjectDetails"/> class.
        /// </summary>
        /// <param name="cubaseApplication"></param>
        /// <param name="cubaseVersion"></param>
        /// <param name="cubaseReleaseDate"></param>
        /// <param name="architecture"></param>
        /// <param name="plugins"></param>
        public ProjectDetails(
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
