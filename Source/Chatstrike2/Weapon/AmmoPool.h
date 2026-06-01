#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "AmmoPool.generated.h"

class ABullet;

// Bullet pooling для оптимизации
CLASS()
class CHATSTRIKE2_API AAmmoPool : public AActor {
	GENERATED_BODY()

public:
	AAmmoPool();

	virtual void BeginPlay() override;

	// Получить пулю из пула
	ABullet* GetBullet();

	// Вернуть пулю в пул
	void ReturnBullet(ABullet* Bullet);

	// Инициализировать пул
	void InitializePool(int32 PoolSize = 500);

	// Получить статистику пула
	int32 GetActiveBulletCount() const { return ActiveBullets; }
	int32 GetPooledBulletCount() const { return PooledBullets.Num(); }

protected:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Pool")
	int32 InitialPoolSize = 500;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Pool")
	TSubclassOf<ABullet> BulletClass;

private:
	TArray<ABullet*> PooledBullets;
	int32 ActiveBullets = 0;

	ABullet* CreateNewBullet();
};