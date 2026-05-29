#pragma once

#include "CoreMinimal.h"
#include "GameFramework/PlayerState.h"
#include "PlayerGameState.generated.h"

UCLASS()
class CHATSTRIKE2_API APlayerGameState : public APlayerState
{
    GENERATED_BODY()

public:
    APlayerGameState();

    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;

    // ========== PLAYER STATS ==========
    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Stats")
    int32 Kills = 0;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Stats")
    int32 Deaths = 0;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Stats")
    int32 Assists = 0;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Stats")
    int32 Level = 1;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Stats")
    int32 Experience = 0;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Stats")
    int32 Money = 0;

    // ========== GAME STATE ==========
    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    float Health = 100.0f;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    float MaxHealth = 100.0f;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    float Armor = 0.0f;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    float MaxArmor = 100.0f;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    int32 CurrentAmmo = 30;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    int32 MaxAmmo = 120;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    bool bIsAlive = true;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "State")
    bool bIsInMatch = false;

    // ========== PLAYER INFO ==========
    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Info")
    FString PlayerNickname = "Player";

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Info")
    int32 PlayerId = -1;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "Info")
    FString PlayerTeam = "Unassigned";

    // ========== MATCH STATS ==========
    UPROPERTY(BlueprintReadWrite, Replicated, Category = "MatchStats")
    float Accuracy = 0.0f;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "MatchStats")
    int32 Headshots = 0;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "MatchStats")
    float DamageDealt = 0.0f;

    UPROPERTY(BlueprintReadWrite, Replicated, Category = "MatchStats")
    float DamageTaken = 0.0f;

    // ========== FUNCTIONS ==========
    UFUNCTION(BlueprintCallable, Category = "Stats")
    void AddKill(int32 Count = 1);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void AddDeath(int32 Count = 1);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void AddAssist(int32 Count = 1);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void AddExperience(int32 Exp);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void AddMoney(int32 Amount);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void UpdateHealth(float NewHealth);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void UpdateArmor(float NewArmor);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void UpdateAmmo(int32 NewAmmo);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    float GetKDRatio() const;

    UFUNCTION(BlueprintCallable, Category = "Stats")
    int32 GetExperienceToNextLevel() const;

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void RecordShot(bool bHit, bool bHeadshot = false, float Damage = 0.0f);

    UFUNCTION(BlueprintCallable, Category = "Stats")
    void SetPlayerTeam(const FString& NewTeam);

    UFUNCTION(BlueprintCallable, Category = "Match")
    void StartMatch();

    UFUNCTION(BlueprintCallable, Category = "Match")
    void EndMatch();

    UFUNCTION(BlueprintCallable, Category = "Match")
    void ResetRound();

private:
    int32 TotalShots = 0;
    int32 HitShots = 0;

    virtual void GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const override;
};