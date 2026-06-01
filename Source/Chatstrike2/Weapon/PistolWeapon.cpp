#include "PistolWeapon.h"

APistolWeapon::APistolWeapon() {
	WeaponType = EWeaponType::Pistol;
	WeaponStats.WeaponName = TEXT("USP-S");
	WeaponStats.BaseDamage = 20.0f;
	WeaponStats.FireRate = 8.0f;
	WeaponStats.Accuracy = 0.85f;
	WeaponStats.RecoilAmount = 0.8f;
	WeaponStats.MagazineSize = 12;
	WeaponStats.MaxAmmo = 120;
	WeaponStats.ReloadTime = 1.5f;
	WeaponStats.CostToPlay = 500.0f;
}

void APistolWeapon::BeginPlay() {
	Super::BeginPlay();
}