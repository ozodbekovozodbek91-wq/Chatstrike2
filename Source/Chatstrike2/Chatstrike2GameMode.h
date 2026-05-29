#pragma once
#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "Chatstrike2GameMode.generated.h"

UCLASS()
class CHATSTRIKE2_API AChatstrike2GameMode : public AGameModeBase
{
    GENERATED_BODY()

public:
    AChatstrike2GameMode();

    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;
};