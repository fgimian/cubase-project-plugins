namespace CubaseProjectPlugins;

/// <summary>
/// Implements the main CLI app for obtaining Cubase plugin details for a project.
/// </summary>
public static class Program
{
    /// <summary>
    /// Displays all plugins used in your Cubase projects along with the Cubase version the project
    /// was created with.
    /// </summary>
    /// <param name="path">The paths to search recursively for Cubase projects.</param>
    /// <param name="configPath">A path to a TOML configuration for the tool.</param>
    /// <returns>The status code of the console application.</returns>
    public static int Main(string[] path, string? configPath = null)
    {
        Config config = new();

        if (!string.IsNullOrEmpty(configPath))
        {
            string configContent;
            try
            {
                configContent = File.ReadAllText(configPath, Encoding.UTF8);
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"Unable to read the config file: {ex.Message}");
                return 1;
            }

            config = TomletMain.To<Config>(configContent);
        }

        if (path.Length == 0)
        {
            Console.Error.WriteLine("You must specify at least one path");
            return 1;
        }

        List<string> projectPaths = new();
        try
        {
            EnumerationOptions options = new()
            {
                RecurseSubdirectories = true
            };

            foreach (string searchPath in path)
            {
                projectPaths.AddRange(Directory.GetFiles(searchPath, "*.cpr", options));
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine(ex.Message);
            return 1;
        }

        SortedSet<Plugin> plugins32 = new();
        SortedSet<Plugin> plugins64 = new();
        SortedSet<Plugin> pluginsAll = new();

        foreach (string projectPath in projectPaths)
        {
            bool skip = false;
            foreach (string pathIgnorePattern in config.PathIgnorePatterns)
            {
                if (pathIgnorePattern.Replace('/', '\\').WildcardMatch(projectPath))
                {
                    skip = true;
                    break;
                }
            }

            if (skip)
            {
                continue;
            }

            // string displayPath = Path.GetRelativePath(path, projectPath);
            ProjectReader reader = new(
                projectBytes: File.ReadAllBytes(projectPath),
                guidIgnores: config.Plugins.GuidIgnores,
                nameIgnores: config.Plugins.NameIgnores);

            bool is64Bit = false;
            try
            {
                ProjectDetails details = reader.GetProjectDetails();

                if (details.Architecture == "WIN64" || details.Architecture == "MAC64 LE")
                {
                    is64Bit = true;
                }

                Console.WriteLine();
                Console.WriteLine(
                    $"{projectPath} [{details.CubaseApplication} {details.CubaseVersion}] " +
                    $"({details.Architecture})");

                if (details.Plugins.Count > 0)
                {
                    Console.WriteLine();
                    foreach (Plugin plugin in details.Plugins)
                    {
                        if (is64Bit)
                        {
                            plugins64.Add(plugin);
                        }
                        else
                        {
                            plugins32.Add(plugin);
                        }

                        pluginsAll.Add(plugin);

                        Console.WriteLine($"    > {plugin.Guid} : {plugin.Name}");
                    }
                }
            }
            catch (InvalidDataException e)
            {
                Console.WriteLine($"{projectPath} - Invalid project file {e}");
            }
        }

        if (plugins32.Count > 0)
        {
            Console.WriteLine();
            Console.WriteLine("Summary of Used Plugins in 32-bit Projects");
            Console.WriteLine();

            foreach (Plugin plugin in plugins32)
            {
                Console.WriteLine($"    > {plugin.Guid} : {plugin.Name}");
            }
        }

        if (plugins64.Count > 0)
        {
            Console.WriteLine();
            Console.WriteLine("Summary of Used Plugins in 64-bit Projects");
            Console.WriteLine();

            foreach (Plugin plugin in plugins64)
            {
                Console.WriteLine($"    > {plugin.Guid} : {plugin.Name}");
            }
        }

        if (pluginsAll.Count > 0)
        {
            Console.WriteLine();
            Console.WriteLine("Summary of Used Plugins in All Projects");
            Console.WriteLine();

            foreach (Plugin plugin in pluginsAll)
            {
                Console.WriteLine($"    > {plugin.Guid} : {plugin.Name}");
            }
        }

        Console.WriteLine();

        return 0;
    }
}
