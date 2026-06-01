# 🔫 STEP 5: Advanced Weapon System

## 📊 Система архитектуры

### Основные компоненты:

#### 1. **WeaponEnums.h**
- `EWeaponType` - 7 типов оружия (AR, Pistol, SMG, Sniper, LMG, Shotgun, Knife)
- `EWeaponState` - состояние оружия (Idle, Firing, Reloading, Aiming, Melee)
- `EAttachmentType` - типы приспособлений (Scope, Silencer, Barrel, Grip, Mag)
- `EAmmoType` - типы амуниции

#### 2. **WeaponStats.h**
- `FWeaponStats` - полная статистика оружия
  - Урон: `BaseDamage`, `HeadshotMultiplier`, `LegMultiplier`
  - Баллистика: `BulletVelocity`, `DamageDropoff`, `MaxDropoffDistance`
  - Стрельба: `FireRate`, `Accuracy`, `RecoilAmount`, `SpreadAmount`
  - Амуниция: `MagazineSize`, `MaxAmmo`, `ReloadTime`
- `FAttachmentStats` - статистика приспособлений
- `FSprayPattern` - паттерн отдачи для каждого оружия

#### 3. **SprayPatternGenerator.h/cpp**
- Генерирует уникальный паттерн отдачи для каждого типа оружия
- AR: вверх + расходится в стороны
- Pistol: минимальная отдача
- SMG: быстрый разброс
- Sniper: очень стабильный
- LMG: прямая линия вверх
- Shotgun: очень широкий разброс
- Функция `GetRecoilOffset()` для получения смещения на каждой пуле

#### 4. **AmmoPool.h/cpp**
- Object pooling для оптимизации производительности
- Переиспользует пули вместо создания новых
- Уменьшает garbage collection
- `GetBullet()` - получить пулю из пула
- `ReturnBullet()` - вернуть пулю в пул

#### 5. **Bullet.h/cpp**
- Класс пули с баллистикой
- `InitializeBullet()` - инициализировать пулю
- `OnHit()` - обработка попадания
- `DealDamage()` - нанести урон с учетом множителей
- Поддержка различных костей для множителей урона

#### 6. **BaseWeapon.h/cpp**
- Базовый класс для всех оружий
- Методы:
  - `StartFiring()` / `StopFiring()` - управление стрельбой
  - `Reload()` / `CompleteReload()` - перезарядка
  - `StartAiming()` / `StopAiming()` - наведение
  - `AttachmentAttach()` / `RemoveAttachment()` - управление приспособлениями
- Делегаты для UI и других систем
- Управление состоянием оружия

#### 7. **Специализированные оружия**
- `RifleWeapon.h/cpp` - M4A1 Assault Rifle
- `PistolWeapon.h/cpp` - USP-S Pistol
- `SMGWeapon.h/cpp` - MP9 Submachine Gun
- `SniperWeapon.h/cpp` - AWP Dragon Lore
- `LMGWeapon.h/cpp` - M249 Light Machine Gun
- `ShotgunWeapon.h/cpp` - XM1014 с множественными пулями
- `KnifeWeapon.h/cpp` - Меле оружие с raycast атакой

#### 8. **WeaponInventory.h/cpp**
- Управление инвентарем оружия игрока
- Переключение между оружием
- Добавление/удаление оружия
- Выбор активного оружия

## 🎯 Ключевые особенности

### ✅ Баллистика:
- Raycast для трассировки пуль
- Damage falloff с расстоянием
- Headshot multiplier (2.5x)
- Leg multiplier (0.75x)
- Armor penetration система (готова для расширения)

### ✅ Spray Pattern:
- Уникальный паттерн для каждого оружия
- Реалистичная отдача
- Стабилизация с контролем игрока
- Случайность для баланса

### ✅ Оптимизация:
- Ammo pooling (500+ пуль в пулу)
- Object reuse вместо создания/удаления
- Минимальный garbage collection

### ✅ Magazine система:
- Реалистичная перезарядка
- Разные времена перезарядки (обычная vs ADS)
- Reserve ammo система
- Ограничение амуниции

### ✅ Attachments:
- Модульная система приспособлений
- Множители урона
- Улучшения точности
- Снижение отдачи
- Шумоподавитель
- Прицелы (zoom)

## 📈 Статистика кода

| Файл | Строк кода | Назначение |
|------|-----------|----------|
| WeaponEnums.h | 35 | Энумы |
| WeaponStats.h | 85 | Структуры данных |
| SprayPatternGenerator | 150+ | Генератор паттернов |
| AmmoPool | 80 | Object pooling |
| Bullet | 120+ | Пули и баллистика |
| BaseWeapon | 280+ | Базовая логика |
| Специализированные оружия | 150+ | Каждое оружие |
| WeaponInventory | 100+ | Управление инвентарем |
| **ИТОГО** | **~1000+** | **Полная система** |

## 🚀 Использование

```cpp
// Создать оружие
ARifleWeapon* Rifle = GetWorld()->SpawnActor<ARifleWeapon>();

// Добавить в инвентарь
Inventory->AddWeapon(Rifle);

// Выбрать оружие
Inventory->SelectWeapon(0);

// Получить текущее оружие
ABaseWeapon* Weapon = Inventory->GetCurrentWeapon();

// Стрельба
Weapon->FireSingleShot(PlayerCharacter, MuzzleLocation, FireDirection);

// Перезарядка
Weapon->Reload();

// Добавить приспособление
FAttachmentStats Scope;
Scope.Type = EAttachmentType::Scope;
Scope.ZoomMultiplier = 2.0f;
Weapon->AttachmentAttach(Scope);
```

## 🎮 Готово для ШАГ 6

Далее можно развивать:
- Визуальные эффекты (muzzle flash, impacts)
- Звуковые эффекты (fire, reload, impact)
- Advanced attachment system
- Weapon skins и customization
- Economy система для покупки оружия
- Сервер-синхронизация (Rust)

---

**Язык**: C++  
**Версия UE**: 5.4+  
**Статус**: ✅ ГОТОВО К ИСПОЛЬЗОВАНИЮ
