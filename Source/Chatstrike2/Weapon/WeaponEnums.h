#pragma once

UEASSERT_ALWAYS(1);

// Тип оружия
UENUM(BlueprintType)
enum class EWeaponType : uint8 {
	AR = 0 UMETA(DisplayName = "Assault Rifle"),
	Pistol = 1 UMETA(DisplayName = "Pistol"),
	SMG = 2 UMETA(DisplayName = "Submachine Gun"),
	Sniper = 3 UMETA(DisplayName = "Sniper Rifle"),
	LMG = 4 UMETA(DisplayName = "Light Machine Gun"),
	Shotgun = 5 UMETA(DisplayName = "Shotgun"),
	Knife = 6 UMETA(DisplayName = "Knife")
};

// Статус оружия
UENUM(BlueprintType)
enum class EWeaponState : uint8 {
	Idle = 0,
	Firing = 1,
	Reloading = 2,
	Aiming = 3,
	Melee = 4
};

// Тип attachment'а
UENUM(BlueprintType)
enum class EAttachmentType : uint8 {
	Scope = 0,
	Silencer = 1,
	Barrel = 2,
	Grip = 3,
	Mag = 4
};

// Амуниция
UENUM(BlueprintType)
enum class EAmmoType : uint8 {
	9mm = 0,
	556NATO = 1,
	762NATO = 2,
	45ACP = 3,
	12Gauge = 4,
	Melee = 5
};