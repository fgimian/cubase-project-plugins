namespace CubaseProjectPlugins
{
    /// <summary>
    /// Captures the Cubase version and all plugins used for a Cubase project.
    /// </summary>
    public class ProjectDetails
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ProjectDetails"/> class.
        /// </summary>
        /// <param name="cubaseApplication">The Cubase application name.</param>
        /// <param name="cubaseVersion">The Cubase version.</param>
        /// <param name="cubaseReleaseDate">The Cubase release date.</param>
        /// <param name="architecture">The system architecture.</param>
        /// <param name="plugins">The plugin names used in the project.</param>
        public ProjectDetails(
            string cubaseApplication,
            string cubaseVersion,
            string cubaseReleaseDate,
            string architecture,
            SortedSet<Plugin> plugins)
        {
            CubaseApplication = cubaseApplication;
            CubaseVersion = cubaseVersion;
            CubaseReleaseDate = cubaseReleaseDate;
            Architecture = architecture;
            Plugins = plugins;
        }

        /// <summary>
        /// Gets or sets the Cubase application name.
        /// </summary>
        public string CubaseApplication { get; set; }

        /// <summary>
        /// Gets or sets the Cubase version used to create the project.
        /// </summary>
        public string CubaseVersion { get; set; }

        /// <summary>
        /// Gets or sets the release date of the Cubase version used.
        /// </summary>
        public string CubaseReleaseDate { get; set; }

        /// <summary>
        /// Gets or sets the system architecture of the Cubase version used.
        /// </summary>
        public string Architecture { get; set; }

        /// <summary>
        /// Gets or sets all the plugins used in the project.
        /// </summary>
        public SortedSet<Plugin> Plugins { get; set; }
    }
}
