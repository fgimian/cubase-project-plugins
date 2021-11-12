namespace CubaseProjectPlugins;

public static class Program
{
    public static void Main(string[] args)
    {
        string[] ignoreNames = new string[]
        {
            "Input Filter",
            "EQ",
            "Standard Panner",
            "Sampler Track",
            "Standard Compressor",
            "EnvelopeShaper",
            "Distroyer",
            "Tube Compressor",
            "Squasher",
            "DeEsser",
            "Magneto II"
        };

        string[] projectPaths = Directory.GetFiles(
            @"C:\Users\Fots\Downloads\projects", "*.cpr", SearchOption.AllDirectories);

        foreach (string projectPath in projectPaths)
        {
            Console.WriteLine(projectPath);

            PluginCounter pluginCounter = new(
                projectBytes: File.ReadAllBytes(projectPath),
                ignoreNames: ignoreNames
            );
            PluginDetails details = pluginCounter.GetCounts();

            Console.WriteLine();
            Console.WriteLine($"{details.CubaseVersion} ({details.Architecture})");
            Console.WriteLine();
            foreach (string plugin in details.Plugins)
            {
                Console.WriteLine(plugin);
            }
            Console.WriteLine();
        }
    }
}
