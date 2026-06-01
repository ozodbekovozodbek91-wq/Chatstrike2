#include "AmmoPool.h"
#include "Bullet.h"

AAmmoPool::AAmmoPool() {
	PrimaryActorTick.bCanEverTick = false;
	RootComponent = CreateDefaultSubobject<USceneComponent>(TEXT("RootComponent"));
}

void AAmmoPool::BeginPlay() {
	Super::BeginPlay();
	InitializePool(InitialPoolSize);
}

void AAmmoPool::InitializePool(int32 PoolSize) {
	for (int32 i = 0; i < PoolSize; i++) {
		ABullet* Bullet = CreateNewBullet();
		if (Bullet) {
			Bullet->SetActorHiddenInGame(true);
			Bullet->SetActorEnableCollision(false);
			PooledBullets.Add(Bullet);
		}
	}
}

ABullet* AAmmoPool::CreateNewBullet() {
	if (!BulletClass) {
		BulletClass = ABullet::StaticClass();
	}

	ABullet* NewBullet = GetWorld()->SpawnActor<ABullet>(BulletClass);
	if (NewBullet) {
		NewBullet->SetOwner(this);
	}
	return NewBullet;
}

ABullet* AAmmoPool::GetBullet() {
	ABullet* Bullet = nullptr;

	if (PooledBullets.Num() > 0) {
		Bullet = PooledBullets.Pop(false);
	} else {
		Bullet = CreateNewBullet();
	}

	if (Bullet) {
		ActiveBullets++;
	}

	return Bullet;
}

void AAmmoPool::ReturnBullet(ABullet* Bullet) {
	if (!Bullet) return;

	Bullet->SetActorHiddenInGame(true);
	Bullet->SetActorEnableCollision(false);
	Bullet->SetActorLocation(FVector::ZeroVector);

	PooledBullets.Add(Bullet);
	ActiveBullets--;
}