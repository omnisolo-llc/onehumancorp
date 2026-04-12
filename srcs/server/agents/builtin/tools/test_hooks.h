#pragma once

#include <glob.h>

namespace ohc::agent {

using GlobFnForTesting =
    int (*)(const char* pattern, int flags,
            int (*errfunc)(const char*, int), glob_t* result);

void SetGlobFnForTesting(GlobFnForTesting glob_fn);
void ResetGlobFnForTesting();

}  // namespace ohc::agent