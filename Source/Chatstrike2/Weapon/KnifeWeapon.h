#pragma once

#include "CoreMinimal.h"
#include "BaseWeapon.h"
#include "KnifeWeapon.generated.h"

// Knife - меле оружие
CLASS()
class CHATSTRIKE2_API AKnifeWeapon : public ABaseWeapon {
	GENERATED_BODY()

public:
	AKnifeWeapon();

	virtual void BeginPlay() override;

	// Удар ножом
	UFUNCTION(BlueprintCallable, Category = "Knife")
	void PerformMeleeAttack(ACharacter* Instigator, FVector AttackOrigin, FVector AttackDirection);

protected:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Knife")
	float MeleeRange = 100.0f;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Knife")
	float MeleeRadius = 50.0f;
};