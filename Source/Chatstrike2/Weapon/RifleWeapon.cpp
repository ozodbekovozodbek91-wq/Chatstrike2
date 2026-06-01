#include "RifleWeapon.h"

ARifleWeapon::ARifleWeapon() {
	WeaponType = EWeaponType::AR;
	WeaponStats.WeaponName = TEXT("M4A1");
	WeaponStats.BaseDamage = 30.0f;
	WeaponStats.FireRate = 10.0f;
	WeaponStats.Accuracy = 0.8f;
	WeaponStats.RecoilAmount = 1.2f;
	WeaponStats.MagazineSize = 30;
	WeaponStats.MaxAmmo = 120;
	WeaponStats.ReloadTime = 2.5f;
	WeaponStats.CostToPlay = 2100.0f;
}

void ARifleWeapon::BeginPlay() {
	Super::BeginPlay();
}

void ARifleWeapon::PlayFireAnimation() {
	Super::PlayFireAnimation();
	// AR-specific effects
}