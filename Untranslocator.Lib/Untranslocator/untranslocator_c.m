#import "untranslocator_c.h"
#import "Untranslocator.h"

#import <Foundation/Foundation.h>
#include <stdlib.h>
#include <string.h>

const char *untranslocator_resolve_path(const char *path_utf8)
{
    if (path_utf8 == NULL)
        return NULL;

    @autoreleasepool
    {
        NSString *inputPath = [NSString stringWithUTF8String:path_utf8];
        if (inputPath == nil)
            return NULL;

        Untranslocator *resolver = [Untranslocator new];
        NSString *resolvedPath = [resolver resolveTranslocatedPath:inputPath];
        const char *resolvedUtf8 = [resolvedPath UTF8String];
        if (resolvedUtf8 == NULL)
            return NULL;

        return strdup(resolvedUtf8);
    }
}

void untranslocator_free_string(const char *owned_str)
{
    if (owned_str != NULL)
        free((void *)owned_str);
}
