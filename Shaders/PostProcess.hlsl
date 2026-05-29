Texture2D<float4> SceneTexture : register(t0);
SamplerState SceneSampler : register(s0);

cbuffer PostProcessParams : register(b0)
{
    float4 ColorTint;
    float BloomIntensity;
    float ContrastAmount;
    float BrightnessAmount;
};

struct VS_OUTPUT
{
    float4 Position : SV_POSITION;
    float2 UV : TEXCOORD0;
};

float4 main(VS_OUTPUT input) : SV_TARGET
{
    float4 sceneColor = SceneTexture.Sample(SceneSampler, input.UV);
    sceneColor.rgb += sceneColor.rgb * BloomIntensity;
    sceneColor.rgb = (sceneColor.rgb - 0.5) * ContrastAmount + 0.5;
    sceneColor.rgb += BrightnessAmount;
    sceneColor.rgb *= ColorTint.rgb;
    return sceneColor;
}
