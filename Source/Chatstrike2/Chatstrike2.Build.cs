using UnrealBuildTool;

public class Chatstrike2 : ModuleRules
{
    public Chatstrike2(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

        PublicDependencyModuleNames.AddRange(new string[] 
        { 
            "Core", 
            "CoreUObject", 
            "Engine", 
            "InputCore",
            "EnhancedInput",
            "Networking",
            "Sockets",
            "OnlineSubsystem"
        });
    }
}