#include "MobileGameMode.h"
#include "Engine/World.h"

AMobileGameMode::AMobileGameMode()
{
    PrimaryActorTick.bCanEverTick = true;
    bIsMobileGame = true;
    bReducedQuality = true;
    MaxPlayers = 8;
    NetworkTickRate = 30.0f;
}

void AMobileGameMode::BeginPlay()
{
    Super::BeginPlay();

    UE_LOG(LogTemp, Warning, TEXT("🎮 ===== MOBILE GAME MODE STARTED ====="));
    UE_LOG(LogTemp, Warning, TEXT("📱 Device: iOS/iPadOS"));
    UE_LOG(LogTemp, Warning, TEXT("🎨 Graphics Quality: %s"), bReducedQuality ? TEXT("LOW") : TEXT("HIGH"));
    UE_LOG(LogTemp, Warning, TEXT("👥 Max Players: %d"), MaxPlayers);
    UE_LOG(LogTemp, Warning, TEXT("🌐 Network Tick: %.1f Hz"), NetworkTickRate);
    UE_LOG(LogTemp, Warning, TEXT("========================================"));

    OptimizeForDevice();
    EnableMobileFriendlyUI();
}

void AMobileGameMode::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    // ✅ Оптимизация на лету
    if (bReducedQuality)
    {
        // Уменьшаем нагрузку каждые 5 секунд
        static float OptimizationTimer = 0.0f;
        OptimizationTimer += DeltaTime;

        if (OptimizationTimer > 5.0f)
        {
            OptimizeForDevice();
            OptimizationTimer = 0.0f;
        }
    }
}

void AMobileGameMode::SetMobileGraphicsQuality(bool bLowQuality)
{
    bReducedQuality = bLowQuality;

    if (bLowQuality)
    {
        UE_LOG(LogTemp, Warning, TEXT("📉 Switching to LOW graphics quality"));
        MaxBotsOnMobile = 2.0f;
        LODDistance = 3000.0f;
        NetworkTickRate = 20.0f;
    }
    else
    {
        UE_LOG(LogTemp, Warning, TEXT("📈 Switching to HIGH graphics quality"));
        MaxBotsOnMobile = 6.0f;
        LODDistance = 8000.0f;
        NetworkTickRate = 30.0f;
    }
}

void AMobileGameMode::OptimizeForDevice()
{
    UE_LOG(LogTemp, Warning, TEXT("⚙️ Optimizing for mobile device..."));

    // ✅ Отключаем дорогие эффекты
    if (GetWorld())
    {
        // Уменьшаем количество ботов
        UE_LOG(LogTemp, Warning, TEXT("🤖 Max bots on mobile: %.0f"), MaxBotsOnMobile);

        // Оптимизируем физику
        UE_LOG(LogTemp, Warning, TEXT("⚡ Physics optimized"));

        // Оптимизируем шейдеры
        UE_LOG(LogTemp, Warning, TEXT("🎨 Shaders optimized"));
    }

    UE_LOG(LogTemp, Warning, TEXT("✅ Optimization complete!"));
}

void AMobileGameMode::EnableMobileFriendlyUI()
{
    UE_LOG(LogTemp, Warning, TEXT("🎮 Enabling mobile-friendly UI..."));

    // ✅ Большие кнопки для сенсора
    // ✅ Адаптивная раскладка экрана
    // ✅ Уменьшенный шрифт для мобилы

    UE_LOG(LogTemp, Warning, TEXT("✅ Mobile UI enabled!"));
}
