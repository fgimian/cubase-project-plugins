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
    /// <param name="ignorePattern">Patterns to ignore when searching for projects.</param>
    /// <returns>The status code of the console application.</returns>
    public static int Main(string[] path, string[] ignorePattern)
    {
        // Create a list of plugins to ignore which are included in Cubase itself.
        string[] ignoreNames = new string[]
        {
            // Special Track Types
            "Sampler Track",

            // Channel Strip
            "Input Filter",
            "Standard Panner",
            "Noise Gate",
            "Standard Compressor",
            "EQ",
            "Tape Saturation",
            "Tube Saturation",
            "Standard Limiter",

            // Plugins
            "AmpSimulator",
            "AutoPan",
            "Bitcrusher",
            "BitCrusher",
            "Brickwall Limiter",
            "Chopper",
            "Chorus",
            "Cloner",
            "Compressor",
            "DaTube",
            "DeEsser",
            "Distortion",
            "Distroyer",
            "DJ-Eq",
            "DualFilter",
            "Embracer",
            "EnvelopeShaper",
            "Expander",
            "Flanger",
            "Frequency",
            "Gate",
            "GEQ-10",
            "GEQ-30",
            "Grungelizer",
            "HALionOne",
            "Imager",
            "Limiter",
            "LoopMash",
            "LoopMash FX",
            "Magneto II",
            "Maximizer",
            "Metalizer",
            "MIDI Gate",
            "MidiGate",
            "Mix6To2",
            "Mix6to2",
            "MixConvert V6",
            "MixConvert",
            "MixConvert-Control Room",
            "MixerDelay",
            "ModMachine",
            "MonoDelay",
            "Monologue",
            "MonoToStereo",
            "MorphFilter",
            "MultibandCompressor",
            "MultibandEnvelopeShaper",
            "MultibandExpander",
            "MultiScope",
            "MultiTap Delay",
            "Mystic",
            "Octaver",
            "Phaser",
            "PingPongDelay",
            "Pitch Correct",
            "Prologue",
            "Quadrafuzz v2",
            "REVelation",
            "REVerence",
            "RingModulator",
            "RoomWorks",
            "RoomWorks SE",
            "Rotary",
            "SMPTEGenerator",
            "SoftClipper",
            "Spector",
            "Squasher",
            "StepFilter",
            "StereoDelay",
            "StereoEnhancer",
            "StudioChorus",
            "StudioEQ",
            "SuperVision",
            "SurroundPan",
            "TestGenerator",
            "ToneBooster",
            "Tonic",
            "Tranceformer",
            "Tremolo",
            "Tube Compressor",
            "Tuner",
            "UV22HR",
            "Vibrato",
            "VintageCompressor",
            "VST AmbiConverter",
            "VST AmbiDecoder",
            "VST Amp Rack",
            "VST Bass Amp",
            "VST Connect CUE Mix",
            "VST Connect Monitor",
            "VST Connect SE",
            "VST MultiPanner",
            "VSTDynamics",
            "WahWah",
        };

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
        catch (Exception e)
        {
            Console.Error.WriteLine(e.Message);
            return 1;
        }

        SortedSet<string> plugins32 = new();
        SortedSet<string> plugins64 = new();
        SortedSet<string> pluginsAll = new();

        foreach (string projectPath in projectPaths)
        {
            bool skip = false;
            foreach (string projectIgnorePattern in ignorePattern)
            {
                if (projectPath.Contains(projectIgnorePattern))
                {
                    skip = true;
                    break;
                }
            }

            if (skip)
            {
                continue;
            }

            //string displayPath = Path.GetRelativePath(path, projectPath);

            ProjectReader reader = new(
                projectBytes: File.ReadAllBytes(projectPath),
                ignoreNames: ignoreNames);

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
                    foreach (string plugin in details.Plugins)
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

                        Console.WriteLine($"    > {plugin}");
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

            foreach (string plugin in plugins32)
            {
                Console.WriteLine($"    > {plugin}");
            }
        }

        if (plugins64.Count > 0)
        {
            Console.WriteLine();
            Console.WriteLine("Summary of Used Plugins in 64-bit Projects");
            Console.WriteLine();

            foreach (string plugin in plugins64)
            {
                Console.WriteLine($"    > {plugin}");
            }
        }

        if (pluginsAll.Count > 0)
        {
            Console.WriteLine();
            Console.WriteLine("Summary of Used Plugins in All Projects");
            Console.WriteLine();

            foreach (string plugin in pluginsAll)
            {
                Console.WriteLine($"    > {plugin}");
            }
        }

        Console.WriteLine();

        return 0;
    }
}
