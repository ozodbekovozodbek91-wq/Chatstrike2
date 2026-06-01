#include "ShotgunWeapon.h"
#include "GameFramework/Character.h"
#include "Bullet.h"
#include "AmmoPool.h"
#include "SprayPatternGenerator.h"

AShotgunWeapon::AShotgunWeapon() {
	WeaponType = EWeaponType::Shotgun;
	WeaponStats.WeaponName = TEXT("XM1014");
	WeaponStats.BaseDamage = 20.0f; // Урон за одну пулю, но их несколько
	WeaponStats.FireRate = 3.0f; // Медленная стрельба
	WeaponStats.Accuracy = 0.6f; // Низкая точность
	WeaponStats.RecoilAmount = 2.5f; // Сильная отдача
	WeaponStats.MagazineSize = 7;
	WeaponStats.MaxAmmo = 32;
	WeaponStats.ReloadTime = 2.0f;
	WeaponStats.CostToPlay = 2200.0f;
}

void AShotgunWeapon::BeginPlay() {
	Super::BeginPlay();
}

void AShotgunWeapon::FireShotgunShot(ACharacter* Instigator, FVector FireOrigin, FVector FireDirection) {
	if (!CanFire()) return;
	if (CurrentAmmo <= 0) {
		Reload();
		return;
	}

	// Стрелять несколькими пулями
	for (int32 i = 0; i < PelletsPerShot; i++) {
		if (AmmoPool) {
			ABullet* Bullet = AmmoPool->GetBullet();
			if (Bullet) {
				// Добавить широкий разброс
				float RandomAngle = FMath::RandRange(-WeaponStats.SpreadAmount * 2, WeaponStats.SpreadAmount * 2);
				float RandomPitch = FMath::RandRange(-WeaponStats.SpreadAmount, WeaponStats.SpreadAmount);

				FMatrix RotMatrix = FRotationMatrix::Make(FQuat::FromAxisAndAngle(FVector::RightVector, FMath::DegreesToRadians(RandomPitch)));
				FVector AdjustedDirection = RotMatrix.TransformVector(FireDirection);

				Bullet->InitializeBullet(Instigator, FireOrigin, AdjustedDirection.GetSafeNormal(), WeaponStats.BaseDamage, WeaponStats.BulletVelocity);
			}
		}
	}

	CurrentAmmo--;
	FireCooldown = 1.0f / WeaponStats.FireRate;
	OnWeaponFired.Broadcast(CurrentAmmo);
	PlayFireAnimation();
}