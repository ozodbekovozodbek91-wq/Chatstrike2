#pragma once

#include "CoreMinimal.h"
#include "BaseWeapon.h"
#include "LMGWeapon.generated.h"

// Light Machine Gun
CLASS()
class CHATSTRIKE2_API ALMGWeapon : public ABaseWeapon {
	GENERATED_BODY()

public:
	ALMGWeapon();

	virtual void BeginPlay() override;
};