#include "SprayPatternGenerator.h"
#include "Math/Vector.h"

ASprayPatternGenerator::ASprayPatternGenerator() {
	PrimaryActorTick.bCanEverTick = false;
}

void ASprayPatternGenerator::BeginPlay() {
	Super::BeginPlay();
}

FSprayPattern ASprayPatternGenerator::GenerateSprayPattern(EWeaponType WeaponType, float Recoil, float Accuracy) {
	FSprayPattern Pattern;

	switch (WeaponType) {
		case EWeaponType::AR:
			Pattern = GenerateARPattern();
			break;
		case EWeaponType::Pistol:
			Pattern = GeneratePistolPattern();
			break;
		case EWeaponType::SMG:
			Pattern = GenerateSMGPattern();
			break;
		case EWeaponType::Sniper:
			Pattern = GenerateSniperPattern();
			break;
		case EWeaponType::LMG:
			Pattern = GenerateLMGPattern();
			break;
		case EWeaponType::Shotgun:
			Pattern = GenerateShotgunPattern();
			break;
		default:
			Pattern = GeneratePistolPattern();
	}

	AddRandomness(Pattern, 1.0f - Accuracy);
	return Pattern;
}

FVector ASprayPatternGenerator::GetRecoilOffset(const FSprayPattern& Pattern, int32 BulletIndex, float Randomness) {
	if (Pattern.Pattern.IsEmpty()) return FVector::ZeroVector;

	int32 PatternIndex = BulletIndex % Pattern.Pattern.Num();
	const FSprayPatternPoint& Point = Pattern.Pattern[PatternIndex];

	float RandomX = FMath::RandRange(-Randomness, Randomness);
	float RandomY = FMath::RandRange(-Randomness, Randomness);

	return FVector(Point.X + RandomX, Point.Y + RandomY, 0.0f);
}

FSprayPattern ASprayPatternGenerator::GenerateARPattern() {
	FSprayPattern Pattern;
	// Паттерн: сначала вверх, потом расходится в стороны
	Pattern.Pattern.Add({0.0f, 0.0f, 0.0f});
	Pattern.Pattern.Add({-0.2f, -0.3f, 0.0f});
	Pattern.Pattern.Add({-0.4f, -0.6f, 0.0f});
	Pattern.Pattern.Add({-0.3f, -0.9f, 0.0f});
	Pattern.Pattern.Add({0.0f, -1.2f, 0.0f});
	Pattern.Pattern.Add({0.3f, -0.9f, 0.0f});
	Pattern.Pattern.Add({0.4f, -0.6f, 0.0f});
	Pattern.Pattern.Add({0.2f, -0.3f, 0.0f});
	Pattern.Pattern.Add({0.5f, 0.0f, 0.0f});
	Pattern.Pattern.Add({0.8f, 0.3f, 0.0f});
	Pattern.RecoilStabilityAt5Shots = 0.75f;
	Pattern.RecoilStabilityAt15Shots = 0.55f;
	Pattern.RecoilStabilityAt30Shots = 0.4f;
	return Pattern;
}

FSprayPattern ASprayPatternGenerator::GeneratePistolPattern() {
	FSprayPattern Pattern;
	// Паттерн: минимальная отдача, немного вверх
	Pattern.Pattern.Add({0.0f, 0.0f, 0.0f});
	Pattern.Pattern.Add({0.1f, -0.2f, 0.0f});
	Pattern.Pattern.Add({-0.1f, -0.3f, 0.0f});
	Pattern.Pattern.Add({0.0f, -0.4f, 0.0f});
	Pattern.Pattern.Add({0.15f, -0.3f, 0.0f});
	Pattern.RecoilStabilityAt5Shots = 0.9f;
	Pattern.RecoilStabilityAt15Shots = 0.85f;
	return Pattern;
}

FSprayPattern ASprayPatternGenerator::GenerateSMGPattern() {
	FSprayPattern Pattern;
	// Паттерн: быстрый разброс вверх и в стороны
	Pattern.Pattern.Add({0.0f, 0.0f, 0.0f});
	Pattern.Pattern.Add({-0.3f, -0.4f, 0.0f});
	Pattern.Pattern.Add({0.2f, -0.8f, 0.0f});
	Pattern.Pattern.Add({-0.4f, -1.0f, 0.0f});
	Pattern.Pattern.Add({0.5f, -1.2f, 0.0f});
	Pattern.Pattern.Add({-0.6f, -0.9f, 0.0f});
	Pattern.Pattern.Add({0.4f, -0.6f, 0.0f});
	Pattern.Pattern.Add({0.8f, -0.3f, 0.0f});
	Pattern.RecoilStabilityAt5Shots = 0.6f;
	Pattern.RecoilStabilityAt15Shots = 0.4f;
	return Pattern;
}

FSprayPattern ASprayPatternGenerator::GenerateSniperPattern() {
	FSprayPattern Pattern;
	// Паттерн: очень минимальный
	Pattern.Pattern.Add({0.0f, 0.0f, 0.0f});
	Pattern.Pattern.Add({0.05f, -0.1f, 0.0f});
	Pattern.Pattern.Add({-0.05f, -0.1f, 0.0f});
	Pattern.RecoilStabilityAt5Shots = 0.95f;
	Pattern.RecoilStabilityAt15Shots = 0.9f;
	return Pattern;
}

FSprayPattern ASprayPatternGenerator::GenerateLMGPattern() {
	FSprayPattern Pattern;
	// Паттерн: прямая вверх с минимальными отклонениями
	Pattern.Pattern.Add({0.0f, 0.0f, 0.0f});
	Pattern.Pattern.Add({0.0f, -0.3f, 0.0f});
	Pattern.Pattern.Add({0.05f, -0.6f, 0.0f});
	Pattern.Pattern.Add({-0.05f, -0.9f, 0.0f});
	Pattern.Pattern.Add({0.0f, -1.2f, 0.0f});
	Pattern.Pattern.Add({0.1f, -1.5f, 0.0f});
	Pattern.Pattern.Add({-0.1f, -1.8f, 0.0f});
	Pattern.RecoilStabilityAt5Shots = 0.8f;
	Pattern.RecoilStabilityAt15Shots = 0.7f;
	Pattern.RecoilStabilityAt30Shots = 0.6f;
	return Pattern;
}

FSprayPattern ASprayPatternGenerator::GenerateShotgunPattern() {
	FSprayPattern Pattern;
	// Паттерн: очень широкий разброс
	Pattern.Pattern.Add({0.0f, 0.0f, 0.0f});
	Pattern.Pattern.Add({-1.0f, -0.5f, 0.0f});
	Pattern.Pattern.Add({1.0f, -0.5f, 0.0f});
	Pattern.Pattern.Add({-0.5f, -1.0f, 0.0f});
	Pattern.Pattern.Add({0.5f, -1.0f, 0.0f});
	Pattern.RecoilStabilityAt5Shots = 0.5f;
	return Pattern;
}

void ASprayPatternGenerator::AddRandomness(FSprayPattern& Pattern, float Amount) {
	for (auto& Point : Pattern.Pattern) {
		Point.X += FMath::RandRange(-Amount, Amount) * 0.5f;
		Point.Y += FMath::RandRange(-Amount, Amount) * 0.5f;
	}
}