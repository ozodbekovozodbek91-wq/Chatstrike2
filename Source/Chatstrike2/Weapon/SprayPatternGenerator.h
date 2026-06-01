#pragma once

#include "CoreMinimal.h"
#include "WeaponStats.h"
#include "GameFramework/Actor.h"
#include "SprayPatternGenerator.generated.h"

CLASS()
class CHATSTRIKE2_API ASprayPatternGenerator : public AActor {
	GENERATED_BODY()

public:
	ASprayPatternGenerator();

	virtual void BeginPlay() override;

	// Генерировать паттерн отдачи для оружия
	FSprayPattern GenerateSprayPattern(EWeaponType WeaponType, float Recoil, float Accuracy);

	// Получить точку отдачи на N-ой пуле
	FVector GetRecoilOffset(const FSprayPattern& Pattern, int32 BulletIndex, float Randomness = 0.1f);

private:
	// AR паттерн - вверх и немного вправо
	FSprayPattern GenerateARPattern();
	// Pistol паттерн - минимальная отдача
	FSprayPattern GeneratePistolPattern();
	// SMG паттерн - широкий разброс
	FSprayPattern GenerateSMGPattern();
	// Sniper паттерн - очень стабильный
	FSprayPattern GenerateSniperPattern();
	// LMG паттерн - прямая линия
	FSprayPattern GenerateLMGPattern();
	// Shotgun паттерн - очень широкий
	FSprayPattern GenerateShotgunPattern();

	// Утилиты
	void AddRandomness(FSprayPattern& Pattern, float Amount);
};