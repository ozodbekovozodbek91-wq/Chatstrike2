#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "PlayerProfileData.generated.h"

USTRUCT(BlueprintType)
struct FPlayerProfile
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    FString PlayerId;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    FString Username;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    int32 Level = 1;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    int32 TotalKills = 0;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    int32 TotalDeaths = 0;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    int32 TotalMatches = 0;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    int32 TotalWins = 0;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    float WinRate = 0.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    float AverageAccuracy = 0.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    int32 TotalPlaytimeMinutes = 0;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    FDateTime LastPlayDate;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    FString FavoriteWeapon;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    float Rank = 0.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    FString Tier = "Unranked";
};

UCLASS()
class CHATSTRIKE2_API UPlayerProfileData : public UDataAsset
{
    GENERATED_BODY()

public:
    UPROPERTY(BlueprintReadWrite, Category = "Profile")
    FPlayerProfile PlayerProfile;

    UFUNCTION(BlueprintCallable, Category = "Profile")
    void SaveProfile();

    UFUNCTION(BlueprintCallable, Category = "Profile")
    void LoadProfile(const FString& PlayerId);

    UFUNCTION(BlueprintCallable, Category = "Profile")
    float CalculateWinRate() const;
};