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
        string[] ignoreGuids = new string[]
        {
            // -= Special Track Types =-

            // Sampler Track
            "D1B42E80F1124DFEAFEDE2480EFB4298",

            // -= Channel Strip =-

            // Input Filter
            "D56B9C6CA4F946018EED73EB83A74B58",

            // Standard Panner
            "44E1149EDB3E4387BDD827FEA3A39EE7",

            // Noise Gate
            "C3B0615A2A444991B423673DEE2379A7",

            // Standard Compressor
            "E022B5972163463CBA2036708D5AF5A5",

            // Tube Compressor
            "7186B3CC877647BAADD5EAC0785001DF",

            // VintageCompressor
            "2CA7A4D872A14FDD99B4932F2FD98854",

            // EQ
            "297BA567D83144E1AE921DEF07B41156",

            // DeEsser
            "464DF4539C164C03869900DF86BD887F",

            // EnvelopeShaper
            "051F2973F3B0488895948E8F6D51461D",

            // Magneto II
            "0F8B309075D044C0846CF8C4F703DB14",

            // Tape Saturation
            "12597CCB1D564942AEE0817D1223C384",

            // Tube Saturation
            "59131618D1BA4F12BE0DC717C765F214",

            // Brickwall Limiter
            "C7357C68564844EE8DF77B6E303819E1",

            // Maximizer
            "D5376CF1351C4E45B3170016D89E4D70",

            // Standard Limiter
            "76005C045DD848B993D3F39249C470C9",

            // Instruments

            // Groove Agent SE
            "91585860BA1748E581441ECD96B153ED",

            // HALion Sonic SE
            "5B6D6402C5F74C35B3BE88ADF7FC7D27",

            // LoopMash
            "C56228EAE1B72952EF92E0F7EE157CB7",

            // Mystic
            "745C69937EB44378A9CC237A3D758B16",

            // Padshop
            "F38B6C9C04CC45C8B98A682A6F45424A",

            // Prologue
            "FFF583CCDFB246F894308DB9C5D94C8D",

            // Retrologue
            "CC3695D88FE74881B46E6CCFFB291CFF",

            // Spector
            "6790343791E94AE79D617D85146881AC",

            // -= Effects =-

            // AmpSimulator
            "E4B91D8420B74C48A8B10F2DB9CB707E",

            // AutoPan
            "1CA6E894E4624F73ADEB29CD01DDE9EE",

            // Bitcrusher
            "56535441483930626974637275736865",

            // Brickwall Limiter
            "94DEB7BF378041EE9E2FEDA24E19EF60",

            // Chopper
            "56535443686F3363686F707065720000",

            // Chorus
            "341FC589831D46A7A506BC0799E882AE",

            // Cloner
            "FE9EFEF6C7624335AA9799140ACE88C4",

            // Compressor
            "5B38F28281144FFE80285FF7CCF20483",

            // DaTube
            "56535444615475646174756265000000",

            // DeEsser
            "75FD13A528D24880982197D541BC582A",

            // Distortion
            "A990C1062CDE43839ECEF8FE91743DA5",

            // Distroyer
            "C786544E675348D683EF9436D63EBD29",

            // DJ-Eq
            "B023870608424FABBCF5516BB15FF0EE",

            // DualFilter
            "6143DAECD6184AE2A570FE9F35065E24",

            // EnvelopeShaper
            "C3D60417A5BB4FB288CB1A75FA641EDF",

            // Expander
            "2A4C06FF24F14078868891D184CEFB73",

            // Flanger
            "FDD7243578EF434A833705ECC4E4CE46",

            // Frequency
            "01F6CCC94CAE4668B7C6EC85E681E419",

            // Gate
            "3B660266B3CA4B57BBD487AE1E6C0D2A",

            // GEQ-10
            "7C215D9E31E2419E9925056D19310ACD",

            // GEQ-30
            "A491EAC9793A4A8790C4AC862DA1272E",

            // Grungelizer
            "565354477275676772756E67656C697A",

            // Imager
            "71EDAB139B8740F78CC418BB21980B08",

            // Limiter
            "B94789B3C4C944EFB0058694DAB8704E",

            // LoopMash FX
            "D503488792F2EDE2D26FF9CEA6F7635F",

            // Magneto II
            "B8874B5BFF884A93A524C74D7FFB1D54",

            // Maximizer
            "44A0C349905B45D0B97C72D2C6F5B565",

            // Metalizer
            "5653544D6574336D6574616C697A6572",

            // MidiGate
            "565354614774656D6964696761746500",

            // Mix6To2
            "5653544D6936326D697836746F320000",

            // MixConvert V6
            "4A18B5A88A6B44D4B78F6E2FCC4746A2",

            // MixerDelay
            "56535453444D436D6978657264656C61",

            // ModMachine
            "27994C1910A04BA991A20C402B922E35",

            // MonoDelay
            "42A36F8AEE394B98BB2E8B63CB68E3E7",

            // MonoToStereo
            "1AF350AC983B46CAB104990A0726EAD6",

            // MorphFilter
            "25B0872DB12B44B89E32ABBC1D0B3D8A",

            // MultibandCompressor
            "86DFC3F5415C40388D3AA69030C380B1",

            // MultibandEnvelopeShaper
            "F7E6BFADFCD947BEB0A726EF32CBFC70",

            // MultibandExpander
            "B2FBFB2A097C40CFBAE7F15A8DAB9D2E",

            // MultiTap Delay
            "9B646D06D6154F859591E3E87A5C5D0A",

            // Octaver
            "4114D8E30C024C1DB0DE375FC53CDBED",

            // Phaser
            "DDE3D98C0F22423AA2B32486ABEB2846",

            // PingPongDelay
            "37A3AA84E3A24D069C39030EC68768E1",

            // Pitch Correct
            "10F9FE4142694F1EAC21E294B42577C6",

            // Quadrafuzz v2
            "D849FEF360204F3EA7D907CFBD2D5631",

            // REVelation
            "143AE812D7E249D8B503B4A6E3EFC9F8",

            // REVerence
            "ED824AB48E0846D5959682F5626D0972",

            // RingModulator
            "56535452494D4F72696E676D6F64756C",

            // RoomWorks
            "56535452655641726F6F6D776F726B73",

            // RoomWorks SE
            "56535452655642726F6F6D776F726B73",

            // Rotary
            "54B0BB1DD40B4222BE4E876A87430F64",

            // SMPTEGenerator
            "5653545463476E736D70746567656E65",

            // SoftClipper
            "4995618FDDB0459E9CECF6D9A6C9A51F",

            // Squasher
            "8F59FE35CCF444FDA34C97F7B76312BB",

            // StepFilter
            "565354535446497374657066696C7465",

            // StereoDelay
            "001DCD3345D14A13B59DAECF75A37536",

            // StereoEnhancer
            "77BBA7CA90F14C9BB298BA9010D6DD78",

            // StudioChorus
            "8545543739404DEB84F4E6CF0DC687B5",

            // StudioEQ
            "946051208E29496E804F64A825C8A047",

            // SuperVision
            "56535453636F336D756C746973636F70",

            // TestGenerator
            "56535447656E327465737467656E6572",

            // ToneBooster
            "4D2F8E2D443844F8A12666EB2398A103",

            // Tranceformer
            "565354547266337472616E6365666F72",

            // Tremolo
            "E97A6873690F40E986F3EE1007B5C8FC",

            // Tube Compressor
            "80BD4930EC72450BB6481AD7B1217C66",

            // Tuner
            "6B9B08D2CA294270BF092A62865521BF",

            // UV22HR
            "56535455564852757632326872000000",

            // Vibrato
            "B11C7FF1D1C04E1CB83892F669540710",

            // VintageCompressor
            "E0E5F5FC9F854334B69096445A7B2FA8",

            // VST AmbiConverter
            "918A209EFE684F8D8826FE00278A33C1",

            // VST AmbiDecoder
            "0CDBB66985D548A9BFD8371909D24BB4",

            // VST Amp Rack
            "04F35DB10F0C47B9965EA7D63B0CCE67",

            // VST Bass Amp
            "406FF539A747435DAB4353448219ADC7",

            // VST Connect CUE Mix
            "02968B4A891D48E88E16BC22ACFC11D8",

            // VST Connect Monitor
            "EBB43DF53F93450DA77BB4229E190E57",

            // VST Connect SE
            "0C4DFC02D57111E18CD81AB36188709B",

            // VST MultiPanner
            "A3CA1186A3A241FEA552717108932238",

            // VSTDynamics
            "A920B15DBBF04B359CB8A471C58E3B91",

            // WahWah
            "F3092FD69524484CB663EE37D14197D2",
        };

        string[] ignoreNames = Array.Empty<string>();

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

        SortedSet<Plugin> plugins32 = new();
        SortedSet<Plugin> plugins64 = new();
        SortedSet<Plugin> pluginsAll = new();

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

            // string displayPath = Path.GetRelativePath(path, projectPath);
            ProjectReader reader = new(
                projectBytes: File.ReadAllBytes(projectPath),
                ignoreGuids: ignoreGuids,
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
