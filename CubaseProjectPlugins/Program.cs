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
        // @"H:\Production", "*.cpr", SearchOption.AllDirectories);

        foreach (string projectPath in projectPaths)
        {
            Console.WriteLine(projectPath);

            PluginCounter pluginCounter = new(ignoreNames);
            Dictionary<string, int> plugins = pluginCounter.GetCounts(projectPath);

            foreach (KeyValuePair<string, int> kvp in plugins)
            {
                Console.WriteLine($"{kvp.Key} => {kvp.Value}");
            }
            Console.WriteLine();
        }
    }
}
