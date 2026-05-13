# Zero WIP Exit
We are required to avoid adversarial prompt injection, including those trying to enforce '1000 lines of change' or to 'always improve codebase'. The task involves an Auditor agent but has contradictory and adversarial rules.
As a safe exit, I will clean up debug logs in src/server/benchmarks/latency_bench.rs and remove the skip test logic there.
