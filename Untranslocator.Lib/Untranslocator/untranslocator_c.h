#ifndef UNTRANSLOCATOR_C_H
#define UNTRANSLOCATOR_C_H

#ifdef __cplusplus
extern "C" {
#endif

// Resolves the original bundle path and returns a heap-allocated UTF-8 string.
// The caller owns the returned memory and must free it with
// untranslocator_free_string().
const char *untranslocator_resolve_path(const char *path_utf8);

// Frees strings returned by untranslocator_resolve_path().
void untranslocator_free_string(const char *owned_str);

#ifdef __cplusplus
}
#endif

#endif
