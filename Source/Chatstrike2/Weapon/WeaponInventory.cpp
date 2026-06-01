#include "WeaponInventory.h"
#include "BaseWeapon.h"
#include "GameFramework/Character.h"

UWeaponInventory::UWeaponInventory() {
	PrimaryComponentTick.bCanEverTick = false;
}

void UWeaponInventory::BeginPlay() {
	Super::BeginPlay();
}

void UWeaponInventory::AddWeapon(ABaseWeapon* Weapon) {
	if (!Weapon) return;

	if (Weapons.Contains(Weapon)) return; // Уже в инвентаре

	Weapons.Add(Weapon);

	if (!CurrentWeapon) {
		SelectWeapon(0);
	}
}

void UWeaponInventory::RemoveWeapon(ABaseWeapon* Weapon) {
	if (!Weapon) return;

	int32 RemoveIndex = Weapons.Find(Weapon);
	if (RemoveIndex != INDEX_NONE) {
		Weapons.RemoveAt(RemoveIndex);

		if (CurrentWeapon == Weapon) {
			if (Weapons.Num() > 0) {
				SelectWeapon(0);
			} else {
				CurrentWeapon = nullptr;
			}
		}
	}
}

void UWeaponInventory::SelectWeapon(int32 WeaponIndex) {
	if (WeaponIndex < 0 || WeaponIndex >= Weapons.Num()) return;

	CurrentWeapon = Weapons[WeaponIndex];
	CurrentWeaponIndex = WeaponIndex;
}

ABaseWeapon* UWeaponInventory::GetWeaponAt(int32 Index) const {
	if (Index < 0 || Index >= Weapons.Num()) return nullptr;
	return Weapons[Index];
}