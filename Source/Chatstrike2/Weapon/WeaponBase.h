#pragma once
#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "WeaponBase.generated.h"

class ACharacter;

UCLASS()
class CHATSTRIKE2_API AWeaponBase : public AActor
{
    GENERATED_BODY()

public:
    AWeaponBase();

    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Weapon")
    float Damage = 25.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Weapon")
    float FireRate = 0.1f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Weapon")
    int32 MaxAmmo = 30;

    UPROPERTY(BlueprintReadWrite, Category = "Weapon")
    int32 CurrentAmmo = 30;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Weapon")
    float ReloadTime = 2.0f;

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    void Fire(const FVector& ShootDirection);

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    void Reload();

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    bool CanFire() const;

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    bool IsReloading() const { return bIsReloading; }

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    float GetReloadProgress() const;

protected:
    UPROPERTY()
    ACharacter* OwnerCharacter;

    float LastFireTime = 0.0f;
    bool bIsReloading = false;
    float ReloadStartTime = 0.0f;

    FTimerHandle ReloadTimerHandle;

    UFUNCTION()
    void OnReloadComplete();
};