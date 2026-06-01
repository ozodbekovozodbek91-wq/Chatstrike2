#pragma once

#include "CoreMinimal.h"
#include "BaseWeapon.h"
#include "RifleWeapon.generated.h"

// Ассолт рифл
CLASS()
class CHATSTRIKE2_API ARifleWeapon : public ABaseWeapon {
	GENERATED_BODY()

public:
	ARifleWeapon();

	virtual void BeginPlay() override;

protected:
	virtual void PlayFireAnimation() override;
};