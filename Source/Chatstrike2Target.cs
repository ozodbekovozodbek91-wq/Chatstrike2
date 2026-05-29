using UnrealBuildTool;

public class Chatstrike2Target : TargetRules
{
    public Chatstrike2Target(TargetInfo Target) : base(Target)
    {
        Type = TargetType.Game;
        DefaultBuildSettings = BuildSettingsVersion.V4;
        IncludeOrderVersion = EngineIncludeOrderVersion.Latest;

        ExtraModuleNames.AddRange(new string[] { "Chatstrike2" });
    }
}