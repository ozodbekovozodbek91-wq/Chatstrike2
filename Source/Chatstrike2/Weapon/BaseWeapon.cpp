#include "BaseWeapon.h"
#include "GameFramework/Character.h"
#include "GameFramework/CharacterMovementComponent.h"
#include "Components/SkeletalMeshComponent.h"
#include "Components/SceneComponent.h"
#include "Bullet.h"
#include "AmmoPool.h"
#include "SprayPatternGenerator.h"
#include "Kismet/GameplayStatics.h"
#include "Sound/SoundCue.h"
#include "Particles/ParticleSystem.h"

ABaseWeapon::ABaseWeapon() {
	PrimaryActorTick.bCanEverTick = true;
	bReplicates = true;

	WeaponMesh = CreateDefaultSubobject<USkeletalMeshComponent>(TEXT("WeaponMesh"));
	RootComponent = WeaponMesh;

	MuzzleSocket = CreateDefaultSubobject<USceneComponent>(TEXT("MuzzleSocket"));
	MuzzleSocket->SetupAttachment(WeaponMesh);
}

void ABaseWeapon::BeginPlay() {
	Super::BeginPlay();

	// Инициализировать ammo pool
	TArray<AActor*> FoundActors;
	UGameplayStatics::GetAllActorsOfClass(GetWorld(), AAmmoPool::StaticClass(), FoundActors);
	if (FoundActors.Num() > 0) {
		AmmoPool = Cast<AAmmoPool>(FoundActors[0]);
	}

	// Инициализировать spray pattern generator
	TArray<AActor*> FoundGenerators;
	UGameplayStatics::GetAllActorsOfClass(GetWorld(), ASprayPatternGenerator::StaticClass(), FoundGenerators);
	if (FoundGenerators.Num() > 0) {
		SprayPatternGenerator = Cast<ASprayPatternGenerator>(FoundGenerators[0]);
	}

	// Сгенерировать spray pattern для этого оружия
	if (SprayPatternGenerator) {
		CurrentSprayPattern = SprayPatternGenerator->GenerateSprayPattern(WeaponType, WeaponStats.Recoil, WeaponStats.Accuracy);
	}

	// Инициализировать амуницию
	CurrentAmmo = WeaponStats.MagazineSize;
	ReserveAmmo = WeaponStats.MaxAmmo - CurrentAmmo;
	TotalAmmo = CurrentAmmo + ReserveAmmo;
}

void ABaseWeapon::Tick(float DeltaTime) {
	Super::Tick(DeltaTime);

	// Обновить таймеры
	if (FireCooldown > 0.0f) {
		FireCooldown -= DeltaTime;
	}

	if (ReloadCooldown > 0.0f) {
		ReloadCooldown -= DeltaTime;
		if (ReloadCooldown <= 0.0f) {
			CompleteReload();
		}
	}

	if (RecoilRecovery > 0.0f) {
		RecoilRecovery -= DeltaTime;
	} else if (FiredBulletsCount > 0) {
		FiredBulletsCount = 0;
	}
}

void ABaseWeapon::FireSingleShot(ACharacter* Instigator, FVector FireOrigin, FVector FireDirection) {
	if (!CanFire()) return;

	if (CurrentAmmo <= 0) {
		Reload();
		return;
	}

	// Создать пулю
	if (AmmoPool) {
		ABullet* Bullet = AmmoPool->GetBullet();
		if (Bullet) {
			// Добавить разброс на основе spray pattern
			FVector SprayOffset = SprayPatternGenerator->GetRecoilOffset(CurrentSprayPattern, FiredBulletsCount, 1.0f - WeaponStats.Accuracy);
			FVector AdjustedDirection = FireDirection + (FireDirection.GetSafeNormal() ^ FVector::UpVector) * SprayOffset.X + FVector::UpVector * SprayOffset.Y;

			// Инициализировать пулю
			Bullet->InitializeBullet(Instigator, FireOrigin, AdjustedDirection.GetSafeNormal(), WeaponStats.BaseDamage, WeaponStats.BulletVelocity);
		}
	}

	// Обновить состояние
	CurrentAmmo--;
	FiredBulletsCount++;
	FireCooldown = 1.0f / WeaponStats.FireRate;
	RecoilRecovery = 0.3f; // Время восстановления от отдачи

	OnWeaponFired.Broadcast(CurrentAmmo);

	// Проиграть звук и эффекты
	PlayFireAnimation();
}

void ABaseWeapon::StartFiring(ACharacter* Instigator, FVector FireOrigin, FVector FireDirection) {
	if (WeaponStats.WeaponType != EWeaponType::Knife) {
		bIsFiring = true;
		SetWeaponState(EWeaponState::Firing);
		FireSingleShot(Instigator, FireOrigin, FireDirection);
	}
}

void ABaseWeapon::StopFiring() {
	bIsFiring = false;
	SetWeaponState(EWeaponState::Idle);
	FiredBulletsCount = 0;
}

void ABaseWeapon::Reload() {
	if (!CanReload()) return;

	SetWeaponState(EWeaponState::Reloading);

	float ReloadTime = bIsAiming ? WeaponStats.ReloadTimeAim : WeaponStats.ReloadTime;
	ReloadCooldown = ReloadTime;

	PlayReloadAnimation();
}

void ABaseWeapon::CompleteReload() {
	int32 RoundsBefore = CurrentAmmo;

	// Рассчитать количество пуль для перезарядки
	int32 AmmoNeeded = WeaponStats.MagazineSize - CurrentAmmo;
	int32 AmmoToAdd = FMath::Min(AmmoNeeded, ReserveAmmo);

	CurrentAmmo += AmmoToAdd;
	ReserveAmmo -= AmmoToAdd;

	SetWeaponState(EWeaponState::Idle);
	OnReload.Broadcast(RoundsBefore, CurrentAmmo);
}

void ABaseWeapon::StartAiming() {
	bIsAiming = true;
	SetWeaponState(EWeaponState::Aiming);
}

void ABaseWeapon::StopAiming() {
	bIsAiming = false;
	SetWeaponState(EWeaponState::Idle);
}

void ABaseWeapon::AttachmentAttach(const FAttachmentStats& Attachment) {
	// Проверить, не установлен ли уже attachment такого типа
	for (int32 i = 0; i < AttachedAttachments.Num(); i++) {
		if (AttachedAttachments[i].Type == Attachment.Type) {
			AttachedAttachments[i] = Attachment; // Заменить
			return;
		}
	}

	// Добавить новый attachment
	AttachedAttachments.Add(Attachment);

	// Применить эффекты attachment'a
	WeaponStats.BaseDamage *= Attachment.DamageMultiplier;
	WeaponStats.Accuracy += Attachment.AccuracyBoost;
	WeaponStats.RecoilAmount -= Attachment.RecoilReduction;
}

void ABaseWeapon::RemoveAttachment(EAttachmentType AttachmentType) {
	for (int32 i = 0; i < AttachedAttachments.Num(); i++) {
		if (AttachedAttachments[i].Type == AttachmentType) {
			// Восстановить начальные значения
			WeaponStats.BaseDamage /= AttachedAttachments[i].DamageMultiplier;
			WeaponStats.Accuracy -= AttachedAttachments[i].AccuracyBoost;
			WeaponStats.RecoilAmount += AttachedAttachments[i].RecoilReduction;

			AttachedAttachments.RemoveAt(i);
			break;
		}
	}
}

bool ABaseWeapon::CanFire() const {
	return FireCooldown <= 0.0f && CurrentState != EWeaponState::Reloading && CurrentAmmo > 0;
}

bool ABaseWeapon::CanReload() const {
	return ReserveAmmo > 0 && CurrentAmmo < WeaponStats.MagazineSize && CurrentState != EWeaponState::Reloading;
}

void ABaseWeapon::SetWeaponState(EWeaponState NewState) {
	if (CurrentState != NewState) {
		CurrentState = NewState;
		OnWeaponStateChanged.Broadcast(NewState);
	}
}

FVector ABaseWeapon::CalculateSpreadOffset() {
	float Spread = WeaponStats.SpreadAmount * (1.0f - WeaponStats.Accuracy);
	if (bIsAiming) {
		Spread *= 0.5f; // Уменьшить разброс при наведении
	}

	float RandomX = FMath::RandRange(-Spread, Spread);
	float RandomY = FMath::RandRange(-Spread, Spread);

	return FVector(RandomX, RandomY, 0.0f);
}

void ABaseWeapon::PlayFireAnimation() {
	// Проиграть звук выстрела
	// UGameplayStatics::PlaySoundAtLocation(GetWorld(), FireSound, MuzzleSocket->GetComponentLocation());

	// Создать эффект вспышки
	// UGameplayStatics::SpawnEmitterAtLocation(GetWorld(), MuzzleFlash, MuzzleSocket->GetComponentLocation());
}

void ABaseWeapon::PlayReloadAnimation() {
	// Проиграть звук перезарядки
	// UGameplayStatics::PlaySoundAtLocation(GetWorld(), ReloadSound, GetActorLocation());
}