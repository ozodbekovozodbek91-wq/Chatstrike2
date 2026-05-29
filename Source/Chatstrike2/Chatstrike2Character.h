#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"
#include "InputActionValue.h"
#include "Chatstrike2Character.generated.h"

class USpringArmComponent;
class UCameraComponent;
class UInputMappingContext;
class UInputAction;
class AWeaponBase;

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
    // ========== CAMERA ==========
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Camera")
    USpringArmComponent* CameraBoom;

    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Camera")
    UCameraComponent* FollowCamera;

    // ========== ENHANCED INPUT SYSTEM ==========
    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputMappingContext* DefaultMappingContext;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputAction* MoveAction;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputAction* LookAction;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputAction* FireAction;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputAction* ReloadAction;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputAction* JumpAction;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputAction* CrouchAction;

    UPROPERTY(EditAnywhere, BlueprintReadOnly, Category = "Input")
    UInputAction* SprintAction;

    // ========== PLAYER STATS ==========
    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float Health = 100.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float MaxHealth = 100.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float Armor = 0.0f;

    UPROPERTY(BlueprintReadWrite, Category = "Player")
    float MaxArmor = 100.0f;

    // ========== WEAPON SYSTEM ==========
    UPROPERTY(BlueprintReadWrite, Category = "Weapon")
    AWeaponBase* CurrentWeapon;

    UPROPERTY(BlueprintReadWrite, Category = "Weapon")
    TArray<AWeaponBase*> Weapons;

    // ========== MOVEMENT STATES ==========
    UPROPERTY(BlueprintReadWrite, Category = "Movement")
    bool bIsSprinting = false;

    UPROPERTY(BlueprintReadWrite, Category = "Movement")
    bool bIsCrouching = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Movement")
    float WalkSpeed = 600.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Movement")
    float SprintSpeed = 1000.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Movement")
    float CrouchSpeed = 300.0f;

    // ========== INPUT CALLBACKS ==========
    void Move(const FInputActionValue& Value);
    void Look(const FInputActionValue& Value);
    void Fire();
    void StopFire();
    void Reload();
    void StartSprint();
    void StopSprint();
    void StartCrouch();
    void StopCrouch();

public:
    // ========== PUBLIC FUNCTIONS ==========
    UFUNCTION(BlueprintCallable, Category = "Health")
    void TakeDamage(float Damage, AActor* DamageCauser);

    UFUNCTION(BlueprintCallable, Category = "Health")
    void Heal(float HealAmount);

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    void EquipWeapon(AWeaponBase* Weapon);

    UFUNCTION(BlueprintCallable, Category = "Weapon")
    AWeaponBase* GetCurrentWeapon() const { return CurrentWeapon; }

    UFUNCTION(BlueprintCallable, Category = "Player")
    float GetHealthPercent() const { return Health / MaxHealth; }

    UFUNCTION(BlueprintCallable, Category = "Player")
    float GetArmorPercent() const { return Armor / MaxArmor; }

    UFUNCTION(BlueprintCallable, Category = "Player")
    bool IsDead() const { return Health <= 0.0f; }

private:
    bool bCanFire = true;
    FTimerHandle FireTimerHandle;
};