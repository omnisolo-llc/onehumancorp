The CI failure is: `System.IO.IOException: No space left on device`.
This is a disk space issue on the GitHub Actions runner, NOT an issue with the code I just committed.

I'll reply to the user explaining that the CI failure is an infrastructure issue and can be resolved by re-running the job, as the code itself has been locally tested and 100% passed `bazelisk test //srcs/server/orchestration/...` as well as `bazelisk test //...`.
