#include "WeaponBase.h"
#include "GameFramework/Character.h"
#include "Kismet/GameplayStatics.h"

AWeaponBase::AWeaponBase()
{
    PrimaryActorTick.bCanEverTick = false;
}

void AWeaponBase::BeginPlay()
{
    Super::BeginPlay();
    OwnerCharacter = Cast<ACharacter>(GetOwner());
}

void AWeaponBase::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);
}

void AWeaponBase::Fire(const FVector& ShootDirection)
{
    if (!CanFire())
        return;

    CurrentAmmo--;
    LastFireTime = GetWorld()->GetTimeSeconds();
    // TODO: Add projectile spawning
}

void AWeaponBase::Reload()
{
    if (bIsReloading || CurrentAmmo == MaxAmmo)
        return;

    bIsReloading = true;
    GetWorld()->GetTimerManager().SetTimer(
        FTimerHandle(),
        [this]()
        {
            CurrentAmmo = MaxAmmo;
            bIsReloading = false;
        },
        2.0f,
        false
    );
}

bool AWeaponBase::CanFire() const
{
    if (bIsReloading || CurrentAmmo <= 0)
        return false;

    float TimeSinceLastFire = GetWorld()->GetTimeSeconds() - LastFireTime;
    return TimeSinceLastFire >= FireRate;
}