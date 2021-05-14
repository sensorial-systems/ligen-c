#pragma once
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
struct Adder {
void* self;
}
float Adder_add(float a, float b);
#ifdef __cplusplus
}
#endif