#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Pawn.h"
#include "InputActionValue.h"
#include "MobileInputComponent.generated.h"

// ✅ МОБИЛЬНОЕ УПРАВЛЕНИЕ ДЛЯ iOS/iPadOS
UCLASS()
class CHATSTRIKE2_API AMobileInputComponent : public APawn
{
    GENERATED_BODY()

public:
    AMobileInputComponent();

    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;
    virtual void SetupPlayerInputComponent(class UInputComponent* PlayerInputComponent) override;

protected:
    // ========== TOUCH AREAS ==========
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    FVector2D LeftThumbstickPosition = FVector2D::ZeroVector;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    FVector2D RightThumbstickPosition = FVector2D::ZeroVector;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    bool bIsLeftThumbstickActive = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    bool bIsRightThumbstickActive = false;

    // ========== BUTTON AREAS ==========
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    bool bFireButtonPressed = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    bool bReloadButtonPressed = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    bool bJumpButtonPressed = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    bool bCrouchButtonPressed = false;

    // ========== TOUCH HANDLING ==========
    void HandleTouchInput(ETouchIndex::Type FingerIndex, FVector Location);
    void ProcessLeftThumbstick(FVector2D TouchLocation);
    void ProcessRightThumbstick(FVector2D TouchLocation);
    void ProcessFireButton(FVector2D TouchLocation);
    void ProcessReloadButton(FVector2D TouchLocation);
    void ProcessJumpButton(FVector2D TouchLocation);
    void ProcessCrouchButton(FVector2D TouchLocation);

    // ========== MOVEMENT & AIM ==========
    void UpdateMovement();
    void UpdateAim();

    // ========== UI ZONES ==========
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|UI")
    FVector2D LeftThumbstickZone = FVector2D(150.0f, 150.0f);

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|UI")
    FVector2D RightThumbstickZone = FVector2D(150.0f, 150.0f);

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|UI")
    FVector2D FireButtonZone = FVector2D(100.0f, 100.0f);

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|UI")
    FVector2D ReloadButtonZone = FVector2D(80.0f, 80.0f);

    // ========== PLAYER STATS ==========
    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float Health = 100.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float MaxHealth = 100.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Player")
    bool bIsAlive = true;

public:
    // ========== PUBLIC FUNCTIONS ==========
    UFUNCTION(BlueprintCallable, Category = "Mobile")
    void OnTouchBegan(ETouchIndex::Type FingerIndex, FVector Location);

    UFUNCTION(BlueprintCallable, Category = "Mobile")
    void OnTouchMoved(ETouchIndex::Type FingerIndex, FVector Location);

    UFUNCTION(BlueprintCallable, Category = "Mobile")
    void OnTouchEnded(ETouchIndex::Type FingerIndex, FVector Location);

    UFUNCTION(BlueprintCallable, Category = "Mobile")
    FVector2D GetMovementInput() const;

    UFUNCTION(BlueprintCallable, Category = "Mobile")
    FVector2D GetLookInput() const;

    UFUNCTION(BlueprintCallable, Category = "Mobile")
    bool IsMoving() const { return !LeftThumbstickPosition.IsNearlyZero(); }
};
