#include "PlayerProfileData.h"
#include "Misc/Paths.h"
#include "HAL/FileManager.h"
#include "Serialization/JsonSerializerBackend.h"
#include "JsonUtilities.h"

void UPlayerProfileData::SaveProfile()
{
    FString SaveDir = FPaths::ProjectSavedDir() / TEXT("PlayerProfiles/");
    IFileManager::Get().MakeDirectory(*SaveDir);

    FString FilePath = SaveDir + PlayerProfile.PlayerId + TEXT(".json");

    FString JsonString;
    FJsonObjectWrapper JsonWrapper;

    // Create JSON from profile
    UE_LOG(LogTemp, Warning, TEXT("Saving profile: %s"), *FilePath);
}

void UPlayerProfileData::LoadProfile(const FString& PlayerId)
{
    FString SaveDir = FPaths::ProjectSavedDir() / TEXT("PlayerProfiles/");
    FString FilePath = SaveDir + PlayerId + TEXT(".json");

    if (FPlatformFileManager::Get().GetPlatformFile().FileExists(*FilePath))
    {
        UE_LOG(LogTemp, Warning, TEXT("Loading profile: %s"), *FilePath);
        PlayerProfile.PlayerId = PlayerId;
    }
    else
    {
        UE_LOG(LogTemp, Warning, TEXT("Profile not found: %s"), *FilePath);
    }
}

float UPlayerProfileData::CalculateWinRate() const
{
    if (PlayerProfile.TotalMatches == 0)
        return 0.0f;

    return (float)PlayerProfile.TotalWins / (float)PlayerProfile.TotalMatches * 100.0f;
}