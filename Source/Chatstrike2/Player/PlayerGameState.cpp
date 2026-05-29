#include "PlayerGameState.h"
#include "Net/UnrealNetwork.h"
#include "GameFramework/PlayerController.h"

APlayerGameState::APlayerGameState()
{
    PrimaryActorTick.bCanEverTick = true;
    PrimaryActorTick.TickInterval = 0.1f;

    bReplicates = true;
    bAlwaysRelevant = true;
}

void APlayerGameState::BeginPlay()
{
    Super::BeginPlay();

    UE_LOG(LogTemp, Warning, TEXT("PlayerGameState initialized for %s"), *PlayerNickname);
}

void APlayerGameState::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);
}

void APlayerGameState::AddKill(int32 Count)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Kills += Count;
        UE_LOG(LogTemp, Warning, TEXT("%s: Kill! Total: %d"), *PlayerNickname, Kills);
    }
}

void APlayerGameState::AddDeath(int32 Count)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Deaths += Count;
        bIsAlive = false;
        UE_LOG(LogTemp, Warning, TEXT("%s: Death! Total: %d"), *PlayerNickname, Deaths);
    }
}

void APlayerGameState::AddAssist(int32 Count)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Assists += Count;
        UE_LOG(LogTemp, Warning, TEXT("%s: Assist! Total: %d"), *PlayerNickname, Assists);
    }
}

void APlayerGameState::AddExperience(int32 Exp)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Experience += Exp;
        UE_LOG(LogTemp, Warning, TEXT("%s: +%d EXP (Total: %d)"), *PlayerNickname, Exp, Experience);

        // Level up check
        int32 ExpPerLevel = 1000;
        int32 NewLevel = (Experience / ExpPerLevel) + 1;
        if (NewLevel > Level)
        {
            Level = NewLevel;
            UE_LOG(LogTemp, Warning, TEXT("%s: LEVEL UP! New Level: %d"), *PlayerNickname, Level);
        }
    }
}

void APlayerGameState::AddMoney(int32 Amount)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Money += Amount;
        UE_LOG(LogTemp, Warning, TEXT("%s: +$%d (Total: $%d)"), *PlayerNickname, Amount, Money);
    }
}

void APlayerGameState::UpdateHealth(float NewHealth)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Health = FMath::Clamp(NewHealth, 0.0f, MaxHealth);

        if (Health <= 0.0f)
        {
            bIsAlive = false;
            UE_LOG(LogTemp, Warning, TEXT("%s: Health depleted!"), *PlayerNickname);
        }
    }
}

void APlayerGameState::UpdateArmor(float NewArmor)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Armor = FMath::Clamp(NewArmor, 0.0f, MaxArmor);
        UE_LOG(LogTemp, Warning, TEXT("%s: Armor updated to %.1f"), *PlayerNickname, Armor);
    }
}

void APlayerGameState::UpdateAmmo(int32 NewAmmo)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        CurrentAmmo = FMath::Clamp(NewAmmo, 0, MaxAmmo);
        UE_LOG(LogTemp, Warning, TEXT("%s: Ammo updated to %d/%d"), *PlayerNickname, CurrentAmmo, MaxAmmo);
    }
}

float APlayerGameState::GetKDRatio() const
{
    if (Deaths == 0)
        return (float)Kills;
    return (float)Kills / (float)Deaths;
}

int32 APlayerGameState::GetExperienceToNextLevel() const
{
    int32 ExpPerLevel = 1000;
    int32 NextLevelExp = (Level) * ExpPerLevel;
    return NextLevelExp - Experience;
}

void APlayerGameState::RecordShot(bool bHit, bool bHeadshot, float Damage)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        TotalShots++;
        if (bHit)
        {
            HitShots++;
            if (bHeadshot)
            {
                Headshots++;
            }
            DamageDealt += Damage;
        }

        Accuracy = (float)HitShots / (float)TotalShots * 100.0f;
        UE_LOG(LogTemp, Warning, TEXT("%s: Shot recorded. Accuracy: %.1f%%, Damage: %.1f"), *PlayerNickname, Accuracy, DamageDealt);
    }
}

void APlayerGameState::SetPlayerTeam(const FString& NewTeam)
{
    if (GetLocalRole() == ROLE_Authority)
    {
        PlayerTeam = NewTeam;
        UE_LOG(LogTemp, Warning, TEXT("%s: Joined team %s"), *PlayerNickname, *PlayerTeam);
    }
}

void APlayerGameState::StartMatch()
{
    if (GetLocalRole() == ROLE_Authority)
    {
        bIsInMatch = true;
        bIsAlive = true;
        Health = MaxHealth;
        UE_LOG(LogTemp, Warning, TEXT("%s: Match started!"), *PlayerNickname);
    }
}

void APlayerGameState::EndMatch()
{
    if (GetLocalRole() == ROLE_Authority)
    {
        bIsInMatch = false;
        UE_LOG(LogTemp, Warning, TEXT("%s: Match ended! K/D: %.2f"), *PlayerNickname, GetKDRatio());
    }
}

void APlayerGameState::ResetRound()
{
    if (GetLocalRole() == ROLE_Authority)
    {
        Health = MaxHealth;
        Armor = 0.0f;
        CurrentAmmo = MaxAmmo;
        bIsAlive = true;
        TotalShots = 0;
        HitShots = 0;
        Accuracy = 0.0f;
        UE_LOG(LogTemp, Warning, TEXT("%s: Round reset!"), *PlayerNickname);
    }
}

void APlayerGameState::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const
{
    Super::GetLifetimeReplicatedProps(OutLifetimeProps);

    DOREPLIFETIME(APlayerGameState, Kills);
    DOREPLIFETIME(APlayerGameState, Deaths);
    DOREPLIFETIME(APlayerGameState, Assists);
    DOREPLIFETIME(APlayerGameState, Level);
    DOREPLIFETIME(APlayerGameState, Experience);
    DOREPLIFETIME(APlayerGameState, Money);
    DOREPLIFETIME(APlayerGameState, Health);
    DOREPLIFETIME(APlayerGameState, MaxHealth);
    DOREPLIFETIME(APlayerGameState, Armor);
    DOREPLIFETIME(APlayerGameState, MaxArmor);
    DOREPLIFETIME(APlayerGameState, CurrentAmmo);
    DOREPLIFETIME(APlayerGameState, MaxAmmo);
    DOREPLIFETIME(APlayerGameState, bIsAlive);
    DOREPLIFETIME(APlayerGameState, bIsInMatch);
    DOREPLIFETIME(APlayerGameState, PlayerNickname);
    DOREPLIFETIME(APlayerGameState, PlayerId);
    DOREPLIFETIME(APlayerGameState, PlayerTeam);
    DOREPLIFETIME(APlayerGameState, Accuracy);
    DOREPLIFETIME(APlayerGameState, Headshots);
    DOREPLIFETIME(APlayerGameState, DamageDealt);
    DOREPLIFETIME(APlayerGameState, DamageTaken);
}