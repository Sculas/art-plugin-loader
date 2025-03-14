#include "apl_wrapper.hpp"

#include <string>

extern "C" void* aplw_alloc_string() {
    return new std::string();
}

extern "C" const char* aplw_string_chars(void* ptr) {
    return static_cast<std::string*>(ptr)->c_str();
}

extern "C" void aplw_free_string(void* ptr) {
    delete static_cast<std::string*>(ptr);
}
