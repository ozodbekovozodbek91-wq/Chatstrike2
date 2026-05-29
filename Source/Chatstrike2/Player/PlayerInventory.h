#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "PlayerInventory.generated.h"

USTRUCT(BlueprintType)
struct FInventoryItem
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadWrite, Category = "Inventory")
    FString ItemId;

    UPROPERTY(BlueprintReadWrite, Category = "Inventory")
    FString ItemName;

    UPROPERTY(BlueprintReadWrite, Category = "Inventory")
    FString ItemType; // Weapon, Armor, Consumable

    UPROPERTY(BlueprintReadWrite, Category = "Inventory")
    int32 Quantity = 1;

    UPROPERTY(BlueprintReadWrite, Category = "Inventory")
    float Rarity = 1.0f; // 0-1 scale
};

UCLASS()
class CHATSTRIKE2_API UPlayerInventory : public UObject
{
    GENERATED_BODY()

public:
    UPROPERTY(BlueprintReadWrite, Category = "Inventory")
    TArray<FInventoryItem> Items;

    UPROPERTY(BlueprintReadWrite, Category = "Inventory")
    int32 InventorySlots = 20;

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    void AddItem(const FInventoryItem& Item);

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    void RemoveItem(const FString& ItemId);

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    FInventoryItem FindItem(const FString& ItemId) const;

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    int32 GetUsedSlots() const { return Items.Num(); }

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    int32 GetAvailableSlots() const { return InventorySlots - Items.Num(); }

    UFUNCTION(BlueprintCallable, Category = "Inventory")
    bool IsFull() const { return Items.Num() >= InventorySlots; }
};