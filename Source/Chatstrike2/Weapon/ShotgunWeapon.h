#pragma once

#include "CoreMinimal.h"
#include "BaseWeapon.h"
#include "ShotgunWeapon.generated.h"

// Shotgun
CLASS()
class CHATSTRIKE2_API AShotgunWeapon : public ABaseWeapon {
	GENERATED_BODY()

public:
	AShotgunWeapon();

	virtual void BeginPlay() override;

	// Дробовик стреляет несколькими пулями одновременно
	void FireShotgunShot(ACharacter* Instigator, FVector FireOrigin, FVector FireDirection);

protected:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Shotgun")
	int32 PelletsPerShot = 8;
};