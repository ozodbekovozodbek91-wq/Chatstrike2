#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "WeaponEnums.h"
#include "WeaponInventory.generated.h"

class ABaseWeapon;
class ACharacter;

// Инвентарь оружия игрока
UCLASS(ClassGroup=(Custom), meta=(BlueprintSpawnableComponent))
class CHATSTRIKE2_API UWeaponInventory : public UActorComponent {
	GENERATED_BODY()

public:
	UWeaponInventory();

	virtual void BeginPlay() override;

	// Добавить оружие в инвентарь
	UFUNCTION(BlueprintCallable, Category = "Inventory")
	void AddWeapon(ABaseWeapon* Weapon);

	// Удалить оружие из инвентаря
	UFUNCTION(BlueprintCallable, Category = "Inventory")
	void RemoveWeapon(ABaseWeapon* Weapon);

	// Выбрать оружие
	UFUNCTION(BlueprintCallable, Category = "Inventory")
	void SelectWeapon(int32 WeaponIndex);

	// Получить текущее оружие
	UFUNCTION(BlueprintCallable, Category = "Inventory")
	ABaseWeapon* GetCurrentWeapon() const { return CurrentWeapon; }

	// Получить оружие по индексу
	UFUNCTION(BlueprintCallable, Category = "Inventory")
	ABaseWeapon* GetWeaponAt(int32 Index) const;

	// Получить количество оружия
	UFUNCTION(BlueprintCallable, Category = "Inventory")
	int32 GetWeaponCount() const { return Weapons.Num(); }

protected:
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Inventory")
	TArray<ABaseWeapon*> Weapons;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Inventory")
	ABaseWeapon* CurrentWeapon = nullptr;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Inventory")
	int32 CurrentWeaponIndex = 0;
};