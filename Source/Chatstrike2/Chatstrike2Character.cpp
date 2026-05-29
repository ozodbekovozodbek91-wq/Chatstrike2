#include "Chatstrike2Character.h"
#include "Camera/CameraComponent.h"
#include "GameFramework/SpringArmComponent.h"
#include "GameFramework/CharacterMovementComponent.h"
#include "EnhancedInputComponent.h"
#include "EnhancedInputSubsystems.h"
#include "InputActionValue.h"
#include "Weapon/WeaponBase.h"
#include "TimerManager.h"
#include "Engine/World.h"

AChatstrike2Character::AChatstrike2Character()
{
    PrimaryActorTick.bCanEverTick = true;

    // Character rotation settings
    bUseControllerRotationPitch = false;
    bUseControllerRotationYaw = false;
    bUseControllerRotationRoll = false;

    // Movement component settings
    GetCharacterMovement()->bOrientRotationToMovement = true;
    GetCharacterMovement()->RotationRate = FRotator(0.0f, 500.0f, 0.0f);
    GetCharacterMovement()->MaxWalkSpeed = WalkSpeed;
    GetCharacterMovement()->MinAnalogWalkSpeed = 20.0f;
    GetCharacterMovement()->MaxWalkSpeedCrouched = CrouchSpeed;

    // Camera boom
    CameraBoom = CreateDefaultSubobject<USpringArmComponent>(TEXT("CameraBoom"));
    CameraBoom->SetupAttachment(RootComponent);
    CameraBoom->TargetArmLength = 0.0f;
    CameraBoom->bUsePawnControlRotation = true;

    // Follow camera
    FollowCamera = CreateDefaultSubobject<UCameraComponent>(TEXT("FollowCamera"));
    FollowCamera->SetupAttachment(CameraBoom, USpringArmComponent::SocketName);
    FollowCamera->bUsePawnControlRotation = false;
}

void AChatstrike2Character::BeginPlay()
{
    Super::BeginPlay();

    // Add Input Mapping Context
    if (APlayerController* PlayerController = Cast<APlayerController>(Controller))
    {
        if (UEnhancedInputLocalPlayerSubsystem* Subsystem =
            PlayerController->GetLocalPlayer()->GetSubsystem<UEnhancedInputLocalPlayerSubsystem>())
        {
            Subsystem->AddMappingContext(DefaultMappingContext, 0);
            UE_LOG(LogTemp, Warning, TEXT("Input Mapping Context added successfully!"));
        }
    }
}

void AChatstrike2Character::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);
}

void AChatstrike2Character::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
    Super::SetupPlayerInputComponent(PlayerInputComponent);

    if (UEnhancedInputComponent* EnhancedInputComponent = 
        Cast<UEnhancedInputComponent>(PlayerInputComponent))
    {
        // Moving
        EnhancedInputComponent->BindAction(MoveAction, ETriggerEvent::Triggered, this, &AChatstrike2Character::Move);

        // Looking
        EnhancedInputComponent->BindAction(LookAction, ETriggerEvent::Triggered, this, &AChatstrike2Character::Look);

        // Firing
        EnhancedInputComponent->BindAction(FireAction, ETriggerEvent::Started, this, &AChatstrike2Character::Fire);
        EnhancedInputComponent->BindAction(FireAction, ETriggerEvent::Completed, this, &AChatstrike2Character::StopFire);

        // Reload
        EnhancedInputComponent->BindAction(ReloadAction, ETriggerEvent::Triggered, this, &AChatstrike2Character::Reload);

        // Jump
        EnhancedInputComponent->BindAction(JumpAction, ETriggerEvent::Started, this, &ACharacter::Jump);
        EnhancedInputComponent->BindAction(JumpAction, ETriggerEvent::Completed, this, &ACharacter::StopJumping);

        // Crouch
        EnhancedInputComponent->BindAction(CrouchAction, ETriggerEvent::Started, this, &AChatstrike2Character::StartCrouch);
        EnhancedInputComponent->BindAction(CrouchAction, ETriggerEvent::Completed, this, &AChatstrike2Character::StopCrouch);

        // Sprint
        EnhancedInputComponent->BindAction(SprintAction, ETriggerEvent::Started, this, &AChatstrike2Character::StartSprint);
        EnhancedInputComponent->BindAction(SprintAction, ETriggerEvent::Completed, this, &AChatstrike2Character::StopSprint);

        UE_LOG(LogTemp, Warning, TEXT("Input Component setup completed!"));
    }
}

void AChatstrike2Character::Move(const FInputActionValue& Value)
{
    const FVector2D MovementVector = Value.Get<FVector2D>();

    if (Controller != nullptr)
    {
        // Find forward direction
        const FRotator Rotation = Controller->GetControlRotation();
        const FRotator YawRotation(0, Rotation.Yaw, 0);

        const FVector ForwardDirection = FRotm(YawRotation).GetUnitAxis(EAxis::X);
        const FVector RightDirection = FRotm(YawRotation).GetUnitAxis(EAxis::Y);

        // Add movement
        AddMovementInput(ForwardDirection, MovementVector.Y);
        AddMovementInput(RightDirection, MovementVector.X);
    }
}

void AChaystrike2Character::Look(const FInputActionValue& Value)
{
    const FVector2D LookAxisVector = Value.Get<FVector2D>();

    if (Controller != nullptr)
    {
        // Add yaw and pitch input to controller
        AddControllerYawInput(LookAxisVector.X);
        AddControllerPitchInput(LookAxisVector.Y);
    }
}

void AChaystrike2Character::Fire()
{
    if (!CurrentWeapon || !bCanFire)
    {
        return;
    }

    FVector ShootDirection = FollowCamera->GetForwardVector();
    CurrentWeapon->Fire(ShootDirection);

    // Set fire rate delay
    bCanFire = false;
    GetWorld()->GetTimerManager().SetTimer(
        FireTimerHandle,
        [this]() { bCanFire = true; },
        CurrentWeapon->FireRate,
        false
    );
}

void AChaystrike2Character::StopFire()
{
    // Could implement automatic fire cutoff here
}

void AChaystrike2Character::Reload()
{
    if (!CurrentWeapon)
    {
        return;
    }

    CurrentWeapon->Reload();
    UE_LOG(LogTemp, Warning, TEXT("Reloading..."));
}

void AChaystrike2Character::StartSprint()
{
    if (GetCharacterMovement())
    {
        bIsSprinting = true;
        GetCharacterMovement()->MaxWalkSpeed = SprintSpeed;
        UE_LOG(LogTemp, Warning, TEXT("Sprint started"));
    }
}

void AChaystrike2Character::StopSprint()
{
    if (GetCharacterMovement())
    {
        bIsSprinting = false;
        GetCharacterMovement()->MaxWalkSpeed = WalkSpeed;
        UE_LOG(LogTemp, Warning, TEXT("Sprint stopped"));
    }
}

void AChaystrike2Character::StartCrouch()
{
    if (!bIsCrouching)
    {
        bIsCrouching = true;
        Crouch();
        UE_LOG(LogTemp, Warning, TEXT("Crouch started"));
    }
}

void AChaystrike2Character::StopCrouch()
{
    if (bIsCrouching)
    {
        bIsCrouching = false;
        UnCrouch();
        UE_LOG(LogTemp, Warning, TEXT("Crouch stopped"));
    }
}

void AChaystrike2Character::TakeDamage(float Damage, AActor* DamageCauser)
{
    // Apply armor reduction
    float DamageReduction = Armor > 0 ? Damage * 0.5f : 0.0f;
    float ActualDamage = Damage - DamageReduction;

    // Reduce armor first
    Armor = FMath::Max(0.0f, Armor - DamageReduction);
    Health = FMath::Max(0.0f, Health - ActualDamage);

    UE_LOG(LogTemp, Warning, TEXT("Player took damage. Health: %.1f, Armor: %.1f"), Health, Armor);

    if (Health <= 0.0f)
    {
        UE_LOG(LogTemp, Warning, TEXT("Player died!"));
        // Trigger death logic here
    }
}

void AChaystrike2Character::Heal(float HealAmount)
{
    Health = FMath::Min(Health + HealAmount, MaxHealth);
    UE_LOG(LogTemp, Warning, TEXT("Player healed. Health: %.1f"), Health);
}

void AChaystrike2Character::EquipWeapon(AWeaponBase* Weapon)
{
    if (Weapon)
    {
        CurrentWeapon = Weapon;
        Weapon->SetOwner(this);
        UE_LOG(LogTemp, Warning, TEXT("Weapon equipped!"));
    }
}