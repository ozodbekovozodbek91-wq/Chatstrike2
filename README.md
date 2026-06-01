# 🎮 Chatstrike2 - Unreal Engine FPS Shooter

Гибридный шутер CS2-подобный на **Unreal Engine 5.x** с многоязычной архитектурой.

## 📋 Стек технологий

| Компонент | Язык | Назначение |
|-----------|------|-----------|
| **Игровой движок** | C++ | Основная логика, механика, оружие |
| **Физика & Античит** | C++ | Высокопроизводительные модули |
| **Сервер** | Rust | Безопасная серверная часть |
| **Веб-платформа** | TypeScript + React | Лаунчер, аккаунты, статистика |
| **AI & Инструменты** | Python | Боты, генерация карт, утилиты |
| **Графика** | HLSL/GLSL | Шейдеры, визуальные эффекты |

## 📁 Структура проекта

```
Chatstrike2/
├── Source/                    # C++ исходный код
│   ├── Chatstrike2/
│   ├── Player/               # Логика игрока
│   ├── Weapon/               # Система оружия
│   ├── Core/                 # Основная логика
│   ├── Physics/              # Модули физики (C++)
│   └── AntiCheat/            # Система защиты
├── Content/                   # Ассеты (модели, текстуры, звуки)
├── Plugins/                   # Плагины UE
├── Server/                    # Rust серверная часть
│   ├── src/
│   └── Cargo.toml
├── Web/                       # TypeScript фронтенд
│   ├── src/
│   ├── package.json
│   └── tsconfig.json
├── Python/                    # Python инструменты
│   ├── ai_bots/
│   ├── map_generator/
│   └── tools/
├── Shaders/                   # HLSL/GLSL шейдеры
├── Docs/                      # Документация
├── .gitignore
├── README.md
└── Chatstrike2.uproject      # UE project file

```

## 🚀 Быстрый старт

### Требования
- **Unreal Engine 5.4+**
- **Visual Studio 2022+** (для C++)
- **Node.js 18+** (для Web)
- **Rust 1.70+** (для сервера)
- **Python 3.10+** (для инструментов)

### Установка

```bash
# Клонируем репозиторий
git clone https://github.com/ozodbekovozodbek91-wq/Chatstrike2.git
cd Chatstrike2

# Генерируем Visual Studio проект
./GenerateProjectFiles.bat

# Открываем в Visual Studio
start Chatstrike2.sln

# Компилируем
# В Visual Studio: Build > Build Solution (Ctrl+Shift+B)
```

## 📂 Основные модули

### C++ - Player System
- Управление персонажем
- Система здоровья
- Система урона
- Синхронизация с сервером

### C++ - Weapon System
- Разные типы оружия
- Баллистика пуль
- Перезарядка и амуниция
- Отдача и точность

### Rust - Server
- Обработка соединений
- Валидация игровых действий
- Anti-cheat логика
- Сохранение статистики

### TypeScript - Launcher
- Авторизация игроков
- Просмотр профиля
- Настройки игры
- Лобби и очереди

### Python - AI & Tools
- Боты для практики
- Процедурная генерация карт
- Анализ логов
- Утилиты разработки

## 🎨 Шейдеры
- Постобработка эффектов
- Система частиц
- Освещение реального времени
- UI шейдеры

## 📜 Лицензия
MIT

## 👨‍💻 Разработка
- **Lead Developer**: ozodbekovozodbek91-wq
- **Engine**: Unreal Engine 5.4+

---

**Статус**: 🔧 проверка игры
