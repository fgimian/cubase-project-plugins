namespace CubaseProjectPlugins;

public static class Program
{
    public static void Main(string[] args)
    {
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
