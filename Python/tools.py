class AIBot:
    def __init__(self, bot_id: str, name: str, difficulty: str):
        self.bot_id = bot_id
        self.name = name
        self.difficulty = difficulty
        self.health = 100
        self.ammo = 30
        self.max_ammo = 120
        self.position = (0, 0, 0)
        self.is_alive = True

    def make_decision(self):
        """Make AI decision based on difficulty"""
        if self.difficulty == "hard":
            return {"action": "hunt", "priority": "high"}
        elif self.difficulty == "normal":
            return {"action": "patrol", "priority": "medium"}
        else:
            return {"action": "move", "direction": (1, 0, 0), "priority": "low"}

    def shoot(self, target_pos) -> bool:
        """✅ ИСПРАВЛЕНИЕ: Проверка амуниции"""
        if self.ammo > 0:
            self.ammo -= 1
            accuracy = self._get_accuracy()
            hit = __import__('random').random() < accuracy
            print(f"🎯 Bot {self.name} shot - Hit: {hit} (Ammo: {self.ammo})")
            return hit
        else:
            print(f"⚠️ Bot {self.name} out of ammo!")
            return False

    def reload(self):
        """✅ ИСПРАВЛЕНИЕ: Перезарядка с логированием"""
        self.ammo = self.max_ammo
        print(f"🔄 Bot {self.name} reloaded! Ammo: {self.ammo}/{self.max_ammo}")

    def take_damage(self, damage: int):
        """✅ ИСПРАВЛЕНИЕ: Обработка урона"""
        self.health -= damage
        if self.health <= 0:
            self.is_alive = False
            print(f"💀 Bot {self.name} died! Health: {self.health}")
        else:
            print(f"💔 Bot {self.name} took {damage} damage. Health: {self.health}")

    def _get_accuracy(self) -> float:
        """✅ ИСПРАВЛЕНИЕ: Точность в зависимости от сложности"""
        accuracy_map = {
            "easy": 0.4,
            "normal": 0.65,
            "hard": 0.9
        }
        return accuracy_map.get(self.difficulty, 0.5)

    def move_to(self, target_pos: tuple) -> dict:
        """✅ ИСПРАВЛЕНИЕ: Безопасное движение"""
        if not self.is_alive:
            return {"status": "error", "message": "Bot is dead"}
        
        import math
        dx = target_pos[0] - self.position[0]
        dy = target_pos[1] - self.position[1]
        dz = target_pos[2] - self.position[2]
        
        distance = math.sqrt(dx**2 + dy**2 + dz**2)
        
        if distance < 1.0:
            return {"status": "arrived", "position": self.position}
        
        speed = 50.0 if self.difficulty == "easy" else 100.0
        move_distance = min(speed * 0.016, distance)  # 0.016s per frame
        
        if distance > 0:
            self.position = (
                self.position[0] + (dx / distance) * move_distance,
                self.position[1] + (dy / distance) * move_distance,
                self.position[2] + (dz / distance) * move_distance
            )
        
        return {
            "status": "moving",
            "position": self.position,
            "distance_remaining": distance - move_distance
        }

class MapGenerator:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.map_data = []

    def generate(self):
        """✅ ИСПРАВЛЕНИЕ: Улучшенная генерация карты"""
        import random
        self.map_data = [[random.randint(0, 1) for _ in range(self.width)] 
                         for _ in range(self.height)]
        return self.map_data

    def validate_position(self, x: int, y: int) -> bool:
        """✅ ИСПРАВЛЕНИЕ: Проверка валидности позиции"""
        if x < 0 or x >= self.width or y < 0 or y >= self.height:
            return False
        return self.map_data[y][x] == 0  # 0 = walkable, 1 = wall

    def get_spawn_points(self, count: int = 4) -> list:
        """✅ ИСПРАВЛЕНИЕ: Получение спавн-поинтов"""
        import random
        spawn_points = []
        attempts = 0
        max_attempts = count * 100
        
        while len(spawn_points) < count and attempts < max_attempts:
            x = random.randint(0, self.width - 1)
            y = random.randint(0, self.height - 1)
            
            if self.validate_position(x, y):
                spawn_points.append((x, y, 0))
            attempts += 1
        
        print(f"📍 Generated {len(spawn_points)} spawn points")
        return spawn_points

if __name__ == "__main__":
    print("🎮 Chatstrike2 Tools v2.0 - Fixed & Improved")
    
    # Тестируем бота
    bot = AIBot("bot_001", "Agent", "hard")
    print(f"✅ Created bot: {bot.name} (Difficulty: {bot.difficulty})")
    
    # Тестируем генератор карты
    gen = MapGenerator(32, 32)
    gen.generate()
    print(f"✅ Generated 32x32 map")
    
    # Тестируем спавн-поинты
    spawn_points = gen.get_spawn_points(4)
    print(f"✅ Spawn points: {spawn_points}")
    
    # Тестируем стрельбу
    bot.shoot((10, 10, 0))
    bot.shoot((10, 10, 0))
    
    # Тестируем урон
    bot.take_damage(25)
    bot.take_damage(75)
