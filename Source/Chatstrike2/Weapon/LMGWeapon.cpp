#include "LMGWeapon.h"

ALMGWeapon::ALMGWeapon() {
	WeaponType = EWeaponType::LMG;
	WeaponStats.WeaponName = TEXT("M249");
	WeaponStats.BaseDamage = 25.0f;
	WeaponStats.FireRate = 12.0f;
	WeaponStats.Accuracy = 0.7f;
	WeaponStats.RecoilAmount = 1.3f;
	WeaponStats.MagazineSize = 100; // Огромный магазин
	WeaponStats.MaxAmmo = 200;
	WeaponStats.ReloadTime = 4.0f; // Долгая перезарядка
	WeaponStats.CostToPlay = 3100.0f;
}

void ALMGWeapon::BeginPlay() {
	Super::BeginPlay();
}