#pragma once

#include "CoreMinimal.h"
#include "WeaponEnums.h"
#include "WeaponStats.generated.h"

// Статистика оружия
USTRUCT(BlueprintType)
struct FWeaponStats {
	GENERATED_BODY()

	// Основные параметры
	FString WeaponName;
	EWeaponType WeaponType;
	EAmmoType AmmoType;

	// Урон
	float BaseDamage = 25.0f;
	float HeadshotMultiplier = 2.5f;
	float LegMultiplier = 0.75f;
	float ArmorPenetration = 0.0f; // 0-1, 1 = сквозь броню
	float DamageDropoff = 0.95f; // За каждый метр
	float MaxDropoffDistance = 5000.0f;

	// Стрельба
	float FireRate = 10.0f; // пули в сек
	float Accuracy = 0.85f; // 0-1
	float RecoilAmount = 1.0f;
	float SpreadAmount = 0.1f; // разброс в градусах
	float AimAccuracyBoost = 2.0f; // улучшение при ADS

	// Амуниция
	int32 MagazineSize = 30;
	int32 MaxAmmo = 120;
	float ReloadTime = 2.5f;
	float ReloadTimeAim = 3.0f;

	// Скорость
	float FireAnimSpeed = 1.0f;
	float RecoilRecoverySpeed = 5.0f;
	float BulletVelocity = 20000.0f;

	// Характеристики
	float Weight = 3.0f;
	float Recoil = 1.0f;
	float RangeAccuracy = 0.8f;
	float CostToPlay = 2100.0f; // Стоимость на покупку
};

// Attachment - приспособление
USTRUCT(BlueprintType)
struct FAttachmentStats {
	GENERATED_BODY()

	FString AttachmentName;
	EAttachmentType Type;
	float DamageMultiplier = 1.0f;
	float AccuracyBoost = 0.0f;
	float RecoilReduction = 0.0f;
	float NoiseReduction = 0.0f; // Для silencer
	float ZoomMultiplier = 1.0f; // Для scope
	float WeightIncrease = 0.0f;
};

// Spray Pattern - паттерн отдачи
USTRUCT(BlueprintType)
struct FSprayPatternPoint {
	GENERATED_BODY()

	float X = 0.0f;
	float Y = 0.0f;
	float Angle = 0.0f;
};

USTRUCT(BlueprintType)
struct FSprayPattern {
	GENERATED_BODY()

	TArray<FSprayPatternPoint> Pattern;
	float RecoilStabilityAt5Shots = 0.8f;
	float RecoilStabilityAt15Shots = 0.6f;
	float RecoilStabilityAt30Shots = 0.5f;
};