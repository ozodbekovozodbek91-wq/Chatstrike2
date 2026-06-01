#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "MobileGameMode.generated.h"

// ✅ МОБИЛЬНЫЙ ИГРОВОЙ РЕЖИМ
UCLASS()
class CHATSTRIKE2_API AMobileGameMode : public AGameModeBase
{
    GENERATED_BODY()

public:
    AMobileGameMode();

    virtual void BeginPlay() override;
    virtual void Tick(float DeltaTime) override;

    // ✅ Мобильные настройки
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    bool bIsMobileGame = true;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    float MobileGravityScale = 1.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile")
    float TouchSensitivity = 1.5f;

    // ✅ Оптимизация графики для мобилы
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|Graphics")
    bool bReducedQuality = true;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|Graphics")
    float MaxBotsOnMobile = 4.0f;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|Graphics")
    float LODDistance = 5000.0f;

    // ✅ Сетевые настройки для мобилы
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|Network")
    int32 MaxPlayers = 8;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mobile|Network")
    float NetworkTickRate = 30.0f;

public:
    UFUNCTION(BlueprintCallable, Category = "Mobile")
    void SetMobileGraphicsQuality(bool bLowQuality);

    UFUNCTION(BlueprintCallable, Category = "Mobile")
    void OptimizeForDevice();

    UFUNCTION(BlueprintCallable, Category = "Mobile")
    void EnableMobileFriendlyUI();
};
