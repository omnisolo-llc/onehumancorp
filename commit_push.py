import os
import subprocess

subprocess.run(["git", "push", "origin", "HEAD:maintainer-fix-telemetry"])
