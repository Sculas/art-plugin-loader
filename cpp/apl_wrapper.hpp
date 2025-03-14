#pragma once

extern "C" void* aplw_alloc_string();
extern "C" const char* aplw_string_chars(void* ptr);
extern "C" void aplw_free_string(void* ptr);
