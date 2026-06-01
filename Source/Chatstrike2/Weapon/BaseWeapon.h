#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "WeaponEnums.h"
#include "WeaponStats.h"
#include "BaseWeapon.generated.h"

class ACharacter;
class ABullet;
class AAmmoPool;
class ASprayPatternGenerator;

DECLARE_DYNAMIC_MULTICAST_DELEGATE_One(FOnWeaponFired, int32, BulletsRemaining);
DECLARE_DYNAMIC_MULTICAST_DELEGATE_Two(FOnReload, int32, RoundsBefore, int32, RoundsAfter);
DECLARE_DYNAMIC_MULTICAST_DELEGATE_One(FOnWeaponStateChanged, EWeaponState, NewState);

// Базовое оружие
CLASS()
class CHATSTRIKE2_API ABaseWeapon : public AActor {
	GENERATED_BODY()

public:
	ABaseWeapon();

	virtual void BeginPlay() override;
	virtual void Tick(float DeltaTime) override;

	// Стрельба
	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void StartFiring(ACharacter* Instigator, FVector FireOrigin, FVector FireDirection);

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void StopFiring();

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void FireSingleShot(ACharacter* Instigator, FVector FireOrigin, FVector FireDirection);

	// Перезарядка
	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void Reload();

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void CompleteReload();

	// Aim
	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void StartAiming();

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void StopAiming();

	// Attachments
	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void AttachmentAttach(const FAttachmentStats& Attachment);

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	void RemoveAttachment(EAttachmentType AttachmentType);

	// Получить информацию
	UFUNCTION(BlueprintCallable, Category = "Weapon")
	const FWeaponStats& GetWeaponStats() const { return WeaponStats; }

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	int32 GetCurrentAmmo() const { return CurrentAmmo; }

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	int32 GetTotalAmmo() const { return TotalAmmo; }

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	EWeaponState GetWeaponState() const { return CurrentState; }

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	bool CanFire() const;

	UFUNCTION(BlueprintCallable, Category = "Weapon")
	bool CanReload() const;

protected:
	// Компоненты
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Weapon")
	class USkeletalMeshComponent* WeaponMesh;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Weapon")
	class USceneComponent* MuzzleSocket;

	// Статистика
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Weapon")
	FWeaponStats WeaponStats;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Weapon")
	EWeaponType WeaponType = EWeaponType::AR;

	// Амуниция
	UPROPERTY(BlueprintReadOnly, Category = "Weapon")
	int32 CurrentAmmo = 30;

	UPROPERTY(BlueprintReadOnly, Category = "Weapon")
	int32 TotalAmmo = 120;

	UPROPERTY(BlueprintReadOnly, Category = "Weapon")
	int32 ReserveAmmo = 90;

	// Состояние
	UPROPERTY(BlueprintReadOnly, Category = "Weapon")
	EWeaponState CurrentState = EWeaponState::Idle;

	UPROPERTY(BlueprintReadOnly, Category = "Weapon")
	bool bIsAiming = false;

	UPROPERTY(BlueprintReadOnly, Category = "Weapon")
	bool bIsFiring = false;

	// Таймеры
	float FireCooldown = 0.0f;
	float ReloadCooldown = 0.0f;
	float RecoilRecovery = 0.0f;

	// Attachments
	UPROPERTY(BlueprintReadOnly, Category = "Weapon")
	TArray<FAttachmentStats> AttachedAttachments;

	// Spray pattern
	FSprayPattern CurrentSprayPattern;
	int32 FiredBulletsCount = 0;

	// Ссылки на системы
	AAmmoPool* AmmoPool = nullptr;
	ASprayPatternGenerator* SprayPatternGenerator = nullptr;

	// Делегаты
	FOnWeaponFired OnWeaponFired;
	FOnReload OnReload;
	FOnWeaponStateChanged OnWeaponStateChanged;

private:
	void SetWeaponState(EWeaponState NewState);
	float CalculateDamage(FVector HitLocation, bool bIsHeadshot = false);
	FVector CalculateSpreadOffset();
	void PlayFireAnimation();
	void PlayReloadAnimation();
};