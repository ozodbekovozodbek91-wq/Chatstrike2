#pragma once

#include "CoreMinimal.h"
#include "BaseWeapon.h"
#include "PistolWeapon.generated.h"

// Пистолет
CLASS()
class CHATSTRIKE2_API APistolWeapon : public ABaseWeapon {
	GENERATED_BODY()

public:
	APistolWeapon();

	virtual void BeginPlay() override;
};