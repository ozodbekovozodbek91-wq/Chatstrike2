#include "SniperWeapon.h"

ASniperWeapon::ASniperWeapon() {
	WeaponType = EWeaponType::Sniper;
	WeaponStats.WeaponName = TEXT("AWP Dragon Lore");
	WeaponStats.BaseDamage = 90.0f; // Очень высокий урон
	WeaponStats.FireRate = 1.5f; // Медленная стрельба
	WeaponStats.Accuracy = 0.95f; // Очень точное
	WeaponStats.RecoilAmount = 2.0f; // Сильная отдача
	WeaponStats.MagazineSize = 10;
	WeaponStats.MaxAmmo = 30;
	WeaponStats.ReloadTime = 3.0f;
	WeaponStats.CostToPlay = 4750.0f; // Самое дорогое
}

void ASniperWeapon::BeginPlay() {
	Super::BeginPlay();
}