#pragma once
#include "CoreMinimal.h"
#include "GameFramework/Character.h"
#include "InputActionValue.h"
#include "Chatstrike2Character.generated.h"

class USpringArmComponent;
class UCameraComponent;

UCLASS()
class CHATSTRIKE2_API AChatstrike2Character : public ACharacter
{
    GENERATED_BODY()

public:
    AChatstrike2Character();

    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;
    virtual void SetupPlayerInputComponent(class UInputComponent* PlayerInputComponent) override;

protected:
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Camera")
    USpringArmComponent* CameraBoom;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Camera")
    UCameraComponent* FollowCamera;

    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float Health = 100.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float MaxHealth = 100.0f;

    void Move(const FInputActionValue& Value);
    void Look(const FInputActionValue& Value);
};