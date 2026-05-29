#include "PlayerInventory.h"

void UPlayerInventory::AddItem(const FInventoryItem& Item)
{
    if (IsFull())
    {
        UE_LOG(LogTemp, Warning, TEXT("Inventory is full!"));
        return;
    }

    Items.Add(Item);
    UE_LOG(LogTemp, Warning, TEXT("Item added: %s (Quantity: %d)"), *Item.ItemName, Item.Quantity);
}

void UPlayerInventory::RemoveItem(const FString& ItemId)
{
    for (int32 i = 0; i < Items.Num(); ++i)
    {
        if (Items[i].ItemId == ItemId)
        {
            UE_LOG(LogTemp, Warning, TEXT("Item removed: %s"), *Items[i].ItemName);
            Items.RemoveAt(i);
            return;
        }
    }

    UE_LOG(LogTemp, Warning, TEXT("Item not found: %s"), *ItemId);
}

FInventoryItem UPlayerInventory::FindItem(const FString& ItemId) const
{
    for (const FInventoryItem& Item : Items)
    {
        if (Item.ItemId == ItemId)
        {
            return Item;
        }
    }

    UE_LOG(LogTemp, Warning, TEXT("Item not found: %s"), *ItemId);
    return FInventoryItem();
}