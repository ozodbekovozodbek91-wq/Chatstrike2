#include "MobileInputComponent.h"
#include "InputActionValue.h"
#include "EnhancedInputComponent.h"
#include "EnhancedInputSubsystems.h"
#include "GameFramework/CharacterMovementComponent.h"

AMobileInputComponent::AMobileInputComponent()
{
    PrimaryActorTick.bCanEverTick = true;
    bUseControllerRotationPitch = false;
    bUseControllerRotationYaw = false;
    bUseControllerRotationRoll = false;

    GetCharacterMovement()->bOrientRotationToMovement = true;
    GetCharacterMovement()->MaxWalkSpeed = 600.0f;
    GetCharacterMovement()->MaxWalkSpeedCrouched = 300.0f;
}

void AMobileInputComponent::BeginPlay()
{
    Super::BeginPlay();

    // ✅ Получаем сенсор тача
    if (APlayerController* PlayerController = Cast<APlayerController>(Controller))
    {
        PlayerController->bShowMouseCursor = false;
        PlayerController->bEnableClickEvents = true;
        PlayerController->bEnableTouchEvents = true;

        UE_LOG(LogTemp, Warning, TEXT("📱 Mobile Input System Initialized!"));
    }
}

void AMobileInputComponent::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    // ✅ Обновляем движение каждый фрейм
    UpdateMovement();
    UpdateAim();
}

void AMobileInputComponent::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
    Super::SetupPlayerInputComponent(PlayerInputComponent);

    // ✅ Тач события
    PlayerInputComponent->BindTouch(EInputEvent::IE_Pressed, this, &AMobileInputComponent::OnTouchBegan);
    PlayerInputComponent->BindTouch(EInputEvent::IE_Released, this, &AMobileInputComponent::OnTouchEnded);

    UE_LOG(LogTemp, Warning, TEXT("✅ Touch input bound!"));
}

// ✅ ТАЧ НАЧАЛАСЬ
void AMobileInputComponent::OnTouchBegan(ETouchIndex::Type FingerIndex, FVector Location)
{
    FVector2D TouchPos = FVector2D(Location.X, Location.Y);

    UE_LOG(LogTemp, Warning, TEXT("👆 Touch Began: %.0f, %.0f | Finger: %d"), TouchPos.X, TouchPos.Y, FingerIndex);

    // Левый стик (движение)
    if (TouchPos.X < 300.0f && TouchPos.Y > FVector2D(GetViewportSize()).Y - 400.0f)
    {
        bIsLeftThumbstickActive = true;
        ProcessLeftThumbstick(TouchPos);
        UE_LOG(LogTemp, Warning, TEXT("🔴 Left Thumbstick ACTIVE"));
    }
    // Правый стик (обзор)
    else if (TouchPos.X > FVector2D(GetViewportSize()).X - 300.0f && TouchPos.Y > FVector2D(GetViewportSize()).Y - 400.0f)
    {
        bIsRightThumbstickActive = true;
        ProcessRightThumbstick(TouchPos);
        UE_LOG(LogTemp, Warning, TEXT("🔵 Right Thumbstick ACTIVE"));
    }
    // Кнопка стрельбы (правый верхний)
    else if (TouchPos.X > FVector2D(GetViewportSize()).X - 150.0f && TouchPos.Y < 150.0f)
    {
        bFireButtonPressed = true;
        UE_LOG(LogTemp, Warning, TEXT("🔫 FIRE BUTTON PRESSED"));
    }
    // Кнопка перезарядки
    else if (TouchPos.X > FVector2D(GetViewportSize()).X - 150.0f && TouchPos.Y > 150.0f && TouchPos.Y < 300.0f)
    {
        bReloadButtonPressed = true;
        UE_LOG(LogTemp, Warning, TEXT("🔄 RELOAD BUTTON PRESSED"));
    }
    // Кнопка прыжка
    else if (TouchPos.X > FVector2D(GetViewportSize()).X - 150.0f && TouchPos.Y > 300.0f && TouchPos.Y < 450.0f)
    {
        bJumpButtonPressed = true;
        Jump();
        UE_LOG(LogTemp, Warning, TEXT("⬆️ JUMP BUTTON PRESSED"));
    }
}

// ✅ ТАЧ ЗАКОНЧИЛАСЬ
void AMobileInputComponent::OnTouchEnded(ETouchIndex::Type FingerIndex, FVector Location)
{
    UE_LOG(LogTemp, Warning, TEXT("👆 Touch Ended: Finger %d"), FingerIndex);

    bIsLeftThumbstickActive = false;
    bIsRightThumbstickActive = false;
    bFireButtonPressed = false;
    bReloadButtonPressed = false;
    bJumpButtonPressed = false;

    LeftThumbstickPosition = FVector2D::ZeroVector;
    RightThumbstickPosition = FVector2D::ZeroVector;
}

// ✅ ОБРАБОТКА ЛЕВОГО СТИКА (ДВИЖЕНИЕ)
void AMobileInputComponent::ProcessLeftThumbstick(FVector2D TouchLocation)
{
    FVector2D ScreenCenter = FVector2D(200.0f, FVector2D(GetViewportSize()).Y - 200.0f);
    FVector2D Offset = TouchLocation - ScreenCenter;

    float Distance = Offset.Length();
    float MaxDistance = 100.0f;

    if (Distance > 0.0f)
    {
        LeftThumbstickPosition = (Offset / Distance) * FMath::Min(Distance, MaxDistance) / MaxDistance;
    }
}

// ✅ ОБРАБОТКА ПРАВОГО СТИКА (ОБЗОР)
void AMobileInputComponent::ProcessRightThumbstick(FVector2D TouchLocation)
{
    FVector2D ScreenSize = FVector2D(GetViewportSize());
    FVector2D ScreenCenter = FVector2D(ScreenSize.X - 200.0f, ScreenSize.Y - 200.0f);
    FVector2D Offset = TouchLocation - ScreenCenter;

    float Distance = Offset.Length();
    float MaxDistance = 100.0f;

    if (Distance > 0.0f)
    {
        RightThumbstickPosition = (Offset / Distance) * FMath::Min(Distance, MaxDistance) / MaxDistance;
    }
}

// ✅ ОБНОВЛЕНИЕ ДВИЖЕНИЯ
void AMobileInputComponent::UpdateMovement()
{
    if (IsMoving())
    {
        FVector Direction = FVector(LeftThumbstickPosition.X, LeftThumbstickPosition.Y, 0.0f).GetSafeNormal();
        AddMovementInput(Direction, 1.0f);
    }
}

// ✅ ОБНОВЛЕНИЕ ОБЗОРА
void AMobileInputComponent::UpdateAim()
{
    if (!RightThumbstickPosition.IsNearlyZero())
    {
        AddControllerYawInput(RightThumbstickPosition.X * 3.0f);
        AddControllerPitchInput(-RightThumbstickPosition.Y * 3.0f);
    }
}

FVector2D AMobileInputComponent::GetMovementInput() const
{
    return LeftThumbstickPosition;
}

FVector2D AMobileInputComponent::GetLookInput() const
{
    return RightThumbstickPosition;
}
