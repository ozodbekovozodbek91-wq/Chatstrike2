#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "WeaponEnums.h"
#include "Bullet.generated.h"

class ACharacter;

DECLARE_DYNAMIC_MULTICAST_DELEGATE_TwoParams(FOnBulletHit, AActor*, HitActor, FVector, ImpactPoint);

// Пуля с баллистикой
CLASS()
class CHATSTRIKE2_API ABullet : public AActor {
	GENERATED_BODY()

public:
	ABullet();

	virtual void BeginPlay() override;
	virtual void Tick(float DeltaTime) override;

	// Инициализировать пулю
	void InitializeBullet(ACharacter* Owner, FVector StartLocation, FVector Direction, float InDamage, float BulletVelocity);

	// Нанести урон
	void DealDamage(AActor* HitActor, FVector HitLocation, float DamageAmount, ACharacter* DamageInstigator);

	// Обработать попадание
	UFUNCTION()
	void OnHit(UPrimitiveComponent* HitComponent, AActor* OtherActor, UPrimitiveComponent* OtherComp, FVector NormalImpulse, const FHitResult& Hit);

protected:
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Bullet")
	UStaticMeshComponent* MeshComponent;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Bullet")
	class USphereComponent* CollisionComponent;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bullet")
	float BaseDamage = 25.0f;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bullet")
	float BulletVelocity = 20000.0f;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bullet")
	float GravityScale = 0.0f; // 0 = нет гравитации

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Bullet")
	float LifeSpan = 30.0f; // Максимальная длительность жизни

	UPROPERTY(BlueprintAssignable, Category = "Bullet")
	FOnBulletHit OnBulletHit;

public:
	FVector CurrentVelocity;
	ACharacter* OwnerCharacter = nullptr;
	float TraveledDistance = 0.0f;

	UFUNCTION(BlueprintCallable, Category = "Bullet")
	void Deactivate();
};