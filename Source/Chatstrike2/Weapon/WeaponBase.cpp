#include "WeaponBase.h"
#include "GameFramework/Character.h"
#include "Kismet/GameplayStatics.h"
#include "TimerManager.h"
#include "Engine/World.h"

AWeaponBase::AWeaponBase()
{
    PrimaryActorTick.bCanEverTick = true;
    PrimaryActorTick.TickInterval = 0.016f; // ~60 FPS
}

void AWeaponBase::BeginPlay()
{
    Super::BeginPlay();
    OwnerCharacter = Cast<ACharacter>(GetOwner());
    
    if (!OwnerCharacter)
    {
        UE_LOG(LogTemp, Warning, TEXT("WeaponBase: Owner is not a Character!"));
    }
}

void AWeaponBase::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);
    // Weapon tick for continuous updates if needed
}

void AWeaponBase::Fire(const FVector& ShootDirection)
{
    if (!CanFire())
    {
        return;
    }

    // Decrease ammo
    CurrentAmmo--;
    LastFireTime = GetWorld()->GetTimeSeconds();

    UE_LOG(LogTemp, Warning, TEXT("Weapon fired! Ammo left: %d"), CurrentAmmo);

    // Check if we need to reload after firing
    if (CurrentAmmo <= 0)
    {
        UE_LOG(LogTemp, Warning, TEXT("Ammo depleted! Auto-reloading..."));
        Reload();
    }
}

void AWeaponBase::Reload()
{
    if (bIsReloading)
    {
        UE_LOG(LogTemp, Warning, TEXT("Already reloading!"));
        return;
    }

    if (CurrentAmmo == MaxAmmo)
    {
        UE_LOG(LogTemp, Warning, TEXT("Magazine is full!"));
        return;
    }

    bIsReloading = true;
    ReloadStartTime = GetWorld()->GetTimeSeconds();

    UE_LOG(LogTemp, Warning, TEXT("Reload started. Duration: %.1f seconds"), ReloadTime);

    // Clear any existing timer
    if (GetWorld()->GetTimerManager().IsTimerActive(ReloadTimerHandle))
    {
        GetWorld()->GetTimerManager().ClearTimer(ReloadTimerHandle);
    }

    // Set new reload timer
    GetWorld()->GetTimerManager().SetTimer(
        ReloadTimerHandle,
        this,
        &AWeaponBase::OnReloadComplete,
        ReloadTime,
        false
    );
}

void AWeaponBase::OnReloadComplete()
{
    CurrentAmmo = MaxAmmo;
    bIsReloading = false;

    UE_LOG(LogTemp, Warning, TEXT("Reload complete! Ammo: %d/%d"), CurrentAmmo, MaxAmmo);
}

bool AWeaponBase::CanFire() const
{
    // Can't fire while reloading
    if (bIsReloading)
    {
        return false;
    }

    // Can't fire without ammo
    if (CurrentAmmo <= 0)
    {
        return false;
    }

    // Check fire rate
    float TimeSinceLastFire = GetWorld()->GetTimeSeconds() - LastFireTime;
    return TimeSinceLastFire >= FireRate;
}

float AWeaponBase::GetReloadProgress() const
{
    if (!bIsReloading)
    {
        return 1.0f; // 100% when not reloading
    }

    float ElapsedTime = GetWorld()->GetTimeSeconds() - ReloadStartTime;
    float Progress = FMath::Clamp(ElapsedTime / ReloadTime, 0.0f, 1.0f);
    return Progress;
}