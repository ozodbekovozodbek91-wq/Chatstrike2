#include "KnifeWeapon.h"
#include "GameFramework/Character.h"
#include "Kismet/GameplayStatics.h"
#include "DrawDebugHelpers.h"

AKnifeWeapon::AKnifeWeapon() {
	WeaponType = EWeaponType::Knife;
	WeaponStats.WeaponName = TEXT("Knife");
	WeaponStats.BaseDamage = 50.0f;
	WeaponStats.FireRate = 2.0f;
	WeaponStats.Accuracy = 1.0f;
	WeaponStats.RecoilAmount = 0.0f;
	WeaponStats.MagazineSize = 0; // Нет амуниции
	WeaponStats.MaxAmmo = 0;
	WeaponStats.ReloadTime = 0.0f;
	WeaponStats.CostToPlay = 0.0f; // Начальное оружие
}

void AKnifeWeapon::BeginPlay() {
	Super::BeginPlay();
}

void AKnifeWeapon::PerformMeleeAttack(ACharacter* Instigator, FVector AttackOrigin, FVector AttackDirection) {
	if (CurrentState == EWeaponState::Reloading) return;

	if (FireCooldown > 0.0f) return;

	// Raycast для проверки попадания
	FHitResult HitResult;
	FVector TraceEnd = AttackOrigin + AttackDirection * MeleeRange;
	FCollisionQueryParams QueryParams;
	QueryParams.AddIgnoredActor(Instigator);

	bool bHit = GetWorld()->LineTraceSingleByChannel(HitResult, AttackOrigin, TraceEnd, ECC_Pawn, QueryParams);

	if (bHit && HitResult.GetActor()) {
		AActor* HitActor = HitResult.GetActor();
		if (HitActor != Instigator) {
			// Нанести урон
			FDamageEvent DamageEvent;
			DamageEvent.DamageTypeClass = UDamageType::StaticClass();
			HitActor->TakeDamage(WeaponStats.BaseDamage, DamageEvent, nullptr, Instigator);
		}
	}

	FireCooldown = 1.0f / WeaponStats.FireRate;
	SetWeaponState(EWeaponState::Melee);
	PlayFireAnimation();
}