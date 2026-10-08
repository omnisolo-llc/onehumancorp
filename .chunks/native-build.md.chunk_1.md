 sudo/Homebrew; host libraries must exist
make init INIT_ARGS=--force            # Reinstall locked npm trees even if stamps match
source target/dev-tools/env.sh         # Use local tools directly; required for Homebrew keg paths
```

Make targets already prepend project-local tool links to PATH. The generated shell snippet does not modify your profile; on macOS source it to expose keg-only PostgreSQL/coreutils/OpenSSL paths to direct commands and shell-based deployment tests. No `.env`, provider key, signing credential, user/group membership or Docker permission is created or modified.

**Docker remains an explicit host prerequisite.** Install/start Docker Engine or Docker Desktop, including Compose v2 and Buildx, and select the intended local context before initialization. `make init` does not make a privileged daemon or change your Docker group membership. It fails if the daemon is unavailable or the effective context is remote; `DOCKER_CONTEXT` takes precedence over `DOCKER_HOST`. Tests create isolated local containers, not a production deployment. Android SDK/NDK, Apple device provisioning, signing/notarization and registry credentials remain release-specific prerequisites rather than requirements for the ordinary host build.

A failed or interrupted install is not recorded as successful. Check an existing `target/dev-tools/.initializing` lock before removing it after an interrupted run; never remove active compiler caches or another user's files. Successful initialization proves environment readiness, **not that application tests pass**. Run the quality gates separately.

The [2026-09-19 measured cleanup record](../research/native_build_measurements_2026-09-19.md) records an **8m 33.94s empty-output backend build**, **1.87s unchanged rerun**, and **31.40s fresh Node build**. Dependency downloads/toolchains were already available; this is not completely cold hosted CI. Keep the **10-minute core backend compilation goal** distinct from the **60-minute full requi