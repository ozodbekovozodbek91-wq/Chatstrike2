#version 450 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texCoord;
layout(location = 3) in vec4 boneWeights;
layout(location = 4) in ivec4 boneIndices;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;
uniform mat4 uBoneTransforms[100];

out VS_OUT {
    vec3 FragPos;
    vec3 Normal;
    vec2 TexCoord;
} vs_out;

void main()
{
    mat4 boneTransform = mat4(0.0);
    for(int i = 0; i < 4; i++) {
        boneTransform += boneWeights[i] * uBoneTransforms[boneIndices[i]];
    }
    
    vec4 animatedPos = boneTransform * vec4(position, 1.0);
    vs_out.FragPos = vec3(uModel * animatedPos);
    vs_out.Normal = normalize(vec3(uModel * vec4(normal, 0.0)));
    vs_out.TexCoord = texCoord;
    
    gl_Position = uProjection * uView * vec4(vs_out.FragPos, 1.0);
}
