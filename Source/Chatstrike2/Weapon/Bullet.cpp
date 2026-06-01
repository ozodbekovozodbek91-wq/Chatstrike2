#include "Bullet.h"
#include "GameFramework/Character.h"
#include "Components/SphereComponent.h"
#include "Components/StaticMeshComponent.h"
#include "GameFramework/ProjectileMovementComponent.h"
#include "PhysicsEngine/BodySetupEnums.h"
#include "Kismet/GameplayStatics.h"

ABullet::ABullet() {
	PrimaryActorTick.bCanEverTick = true;
	bReplicates = true;

	// Collision
	CollisionComponent = CreateDefaultSubobject<USphereComponent>(TEXT("CollisionComponent"));
	CollisionComponent->InitSphereRadius(2.0f);
	CollisionComponent->SetCollisionEnabled(ECC_WorldDynamic);
	CollisionComponent->SetCollisionObjectType(ECC_WorldDynamic);
	CollisionComponent->SetCollisionResponseToAllChannels(ECR_Block);
	CollisionComponent->OnComponentHit.AddDynamic(this, &ABullet::OnHit);
	RootComponent = CollisionComponent;

	// Mesh
	MeshComponent = CreateDefaultSubobject<UStaticMeshComponent>(TEXT("MeshComponent"));
	MeshComponent->SetupAttachment(RootComponent);
	MeshComponent->SetCollisionEnabled(ECC_NoCollision);
}

void ABullet::BeginPlay() {
	Super::BeginPlay();
	SetLifeSpan(LifeSpan);
}

void ABullet::Tick(float DeltaTime) {
	Super::Tick(DeltaTime);

	if (!CurrentVelocity.IsZero()) {
		FVector NewLocation = GetActorLocation() + CurrentVelocity * DeltaTime;
		TraveledDistance += (NewLocation - GetActorLocation()).Length();

		// Применить гравитацию если нужна
		if (GravityScale > 0.0f) {
			CurrentVelocity.Z -= 980.0f * GravityScale * DeltaTime;
		}

		SetActorLocation(NewLocation);

		// Ориентировать по направлению полета
		FRotator NewRotation = CurrentVelocity.Rotation();
		SetActorRotation(NewRotation);
	}
}

void ABullet::InitializeBullet(ACharacter* Owner, FVector StartLocation, FVector Direction, float InDamage, float InBulletVelocity) {
	OwnerCharacter = Owner;
	BaseDamage = InDamage;
	BulletVelocity = InBulletVelocity;
	CurrentVelocity = Direction.GetSafeNormal() * BulletVelocity;
	TraveledDistance = 0.0f;

	SetActorLocation(StartLocation);
	SetActorRotation(Direction.Rotation());
	SetActorHiddenInGame(false);
	SetActorEnableCollision(true);
}

void ABullet::OnHit(UPrimitiveComponent* HitComponent, AActor* OtherActor, UPrimitiveComponent* OtherComp, FVector NormalImpulse, const FHitResult& Hit) {
	if (OtherActor && OtherActor != OwnerCharacter && OtherActor != this) {
		FVector ImpactPoint = Hit.ImpactPoint;

		// Вычи��лить урон
		float DamageAmount = BaseDamage;

		// Проверить попадание в голову
		if (Hit.BoneName == FName(TEXT("Head"))) {
			DamageAmount *= 2.5f; // Headshot multiplier
		}

		// Проверить попадание в ноги
		if (Hit.BoneName == FName(TEXT("Foot_L")) || Hit.BoneName == FName(TEXT("Foot_R"))) {
			DamageAmount *= 0.75f; // Leg multiplier
		}

		DealDamage(OtherActor, ImpactPoint, DamageAmount, OwnerCharacter);
		OnBulletHit.Broadcast(OtherActor, ImpactPoint);
		Deactivate();
	}
}

void ABullet::DealDamage(AActor* HitActor, FVector HitLocation, float DamageAmount, ACharacter* DamageInstigator) {
	if (!HitActor) return;

	FDamageEvent DamageEvent;
	DamageEvent.DamageTypeClass = UDamageType::StaticClass();

	HitActor->TakeDamage(DamageAmount, DamageEvent, nullptr, DamageInstigator);
}

void ABullet::Deactivate() {
	CurrentVelocity = FVector::ZeroVector;
	SetActorHiddenInGame(true);
	SetActorEnableCollision(false);
	SetActorLocation(FVector(-100000.0f, -100000.0f, -100000.0f)); // Отправить далеко
}