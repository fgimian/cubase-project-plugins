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
    /// <param name="path">The path to search recursively for Cubase projects</param>
    /// <returns></returns>
    public static int Main(string path)
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

        if (path == "")
        {
            Console.Error.WriteLine("You must specify a path");
            return 1;
        }

        string[] projectPaths;
        try
        {
            projectPaths = Directory.GetFiles(path, "*.cpr", SearchOption.AllDirectories);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine(e.Message); ;
            return 1;
        }

        foreach (string projectPath in projectPaths)
        {
            string displayPath = Path.GetRelativePath(path, projectPath);

            ProjectReader reader = new(
                projectBytes: File.ReadAllBytes(projectPath),
                ignoreNames: ignoreNames
            );

            Console.WriteLine();
            try
            {
                ProjectDetails details = reader.GetProjectDetails();

                Console.WriteLine(
                    $"{displayPath} [{details.CubaseApplication} {details.CubaseVersion}] " +
                    $"({details.Architecture})");

                if (details.Plugins.Count > 0)
                {
                    Console.WriteLine();
                    foreach (string plugin in details.Plugins)
                    {
                        Console.WriteLine($"    > {plugin}");
                    }
                }
            }
            catch (InvalidDataException e)
            {
                Console.WriteLine($"{displayPath} - Invalid project file {e}");
            }
        }
        Console.WriteLine();

        return 0;
    }
}
