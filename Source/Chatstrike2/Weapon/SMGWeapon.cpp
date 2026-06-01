#include "SMGWeapon.h"

ASMGWeapon::ASMGWeapon() {
	WeaponType = EWeaponType::SMG;
	WeaponStats.WeaponName = TEXT("MP9");
	WeaponStats.BaseDamage = 18.0f;
	WeaponStats.FireRate = 20.0f; // Быстрая стрельба
	WeaponStats.Accuracy = 0.75f;
	WeaponStats.RecoilAmount = 1.5f;
	WeaponStats.MagazineSize = 25;
	WeaponStats.MaxAmmo = 150;
	WeaponStats.ReloadTime = 1.8f;
	WeaponStats.CostToPlay = 1200.0f;
}

void ASMGWeapon::BeginPlay() {
	Super::BeginPlay();
}