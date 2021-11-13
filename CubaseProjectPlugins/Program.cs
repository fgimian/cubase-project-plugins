namespace CubaseProjectPlugins;

public static class Program
{
    public static void Main(string[] args)
    {
        string[] ignoreNames = new string[]
        {
            "Input Filter",
            "Standard Panner",
            "EQ",
            "Sampler Track",
            "Standard Compressor",
            // "Noise Gate",  // TODO check
            // "Tape Saturation",  // TODO check
            // "Tube Saturation",  // TODO check
            // "Standard Limiter",
            // Plugins
            "AmpSimulator",
            "AutoPan",
            "Bitcrusher",
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
            "EnvelopeShaper",
            "Expander",
            "Flanger",
            "Frequency",
            "Gate",
            "GEQ-10",
            "GEQ-30",
            "Grungelizer",
            "Imager",
            "Limiter",
            "LoopMash",
            "LoopMash FX",
            "Magneto II",
            "Maximizer",
            "Metalizer",
            "MidiGate",
            "Mix6To2",
            "MixConvert V6",
            "MixerDelay",
            "ModMachine",
            "MonoDelay",
            "MonoToStereo",
            "MorphFilter",
            "MultibandCompressor",
            "MultibandEnvelopeShaper",
            "MultibandExpander",
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
            "TestGenerator",
            "ToneBooster",
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
            Console.WriteLine($"{details.CubaseApplication} {details.CubaseVersion} ({details.Architecture})");

            if (details.Plugins.Count > 0)
            {
                Console.WriteLine();
                foreach (string plugin in details.Plugins)
                {
                    Console.WriteLine(plugin);
                }
            }

            Console.WriteLine();
        }
    }
}
