#pragma once

#include "CoreMinimal.h"
#include "BaseWeapon.h"
#include "SMGWeapon.generated.h"

// Submachine Gun
CLASS()
class CHATSTRIKE2_API ASMGWeapon : public ABaseWeapon {
	GENERATED_BODY()

public:
	ASMGWeapon();

	virtual void BeginPlay() override;
};