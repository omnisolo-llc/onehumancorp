#!/bin/bash
cat << 'PATCH' > interop_build.patch
--- srcs/server/interop/BUILD.bazel
+++ srcs/server/interop/BUILD.bazel
@@ -5,7 +5,10 @@
     srcs = [
         "autogen_adapter.go",
         "crewai_adapter.go",
+        "distributed_lock.go",
         "ironclaw_adapter.go",
+        "mesh.go",
+        "mesh_telemetry.go",
         "openclaw_adapter.go",
         "semantickernel_adapter.go",
         "types.go",
@@ -17,7 +20,10 @@
 go_test(
     name = "interop_test",
     srcs = [
+        "distributed_lock_test.go",
+        "distributed_lock_redis_test.go",
         "ironclaw_adapter_test.go",
+        "mesh_test.go",
         "swarm_test.go",
         "types_test.go",
     ],
PATCH
patch srcs/server/interop/BUILD.bazel interop_build.patch
