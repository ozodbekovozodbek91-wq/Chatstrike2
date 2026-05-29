#include "Chatstrike2GameMode.h"

AChatstrike2GameMode::AChatstrike2GameMode()
{
    DefaultPawnClass = ACharacter::StaticClass();
    PrimaryActorTick.bCanEverTick = true;
}

void AChatstrike2GameMode::BeginPlay()
{
    Super::BeginPlay();
    if (GEngine)
    {
        GEngine->AddOnScreenDebugMessage(-1, 5.0f, FColor::Green, TEXT("Chatstrike2 Started!"));
    }
}

void AChatstrike2GameMode::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);
}