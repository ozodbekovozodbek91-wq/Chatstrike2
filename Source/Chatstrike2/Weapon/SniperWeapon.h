#pragma once

#include "CoreMinimal.h"
#include "BaseWeapon.h"
#include "SniperWeapon.generated.h"

// Sniper rifle
CLASS()
class CHATSTRIKE2_API ASniperWeapon : public ABaseWeapon {
	GENERATED_BODY()

public:
	ASniperWeapon();

	virtual void BeginPlay() override;
};