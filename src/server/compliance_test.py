import os
import sys

def check_pii():
    violations = []
    keywords = ["println!", "eprintln!", "info!", "error!", "warn!", "debug!", "tracing::"]
    pii_words = ["tenant_id", "org_id", "session_data", "session_id", "payload", "email", "password", "pii"]

    checked = 0
    # Bazel runs tests in a sandbox. To access the source code, we use RUNFILES_DIR.
    # We included '**/*.rs' in data, which will map them into runfiles.
    runfiles_dir = os.environ.get("RUNFILES_DIR", ".")

    for root, _, files in os.walk(runfiles_dir):
        # Prevent checking external bazel dependencies
        if "external" in root.split(os.sep):
            continue

        for f in files:
            if f.endswith(".rs"):
                checked += 1
                filepath = os.path.join(root, f)
                try:
                    with open(filepath, 'r', encoding='utf-8') as file:
                        for i, line in enumerate(file):
                            lower_line = line.lower()
                            if any(kw in lower_line for kw in keywords):
                                if any(pw in lower_line for pw in pii_words):
                                    violations.append(f"{filepath}:{i+1}: {line.strip()}")
                except Exception as e:
                    print(f"Failed to read {filepath}: {e}")

    if checked < 10:
        print(f"Error: Only checked {checked} files. Sandbox might be misconfigured.")
        sys.exit(1)

    if violations:
        print("PII LEAKAGE DETECTED:")
        for v in violations:
            print(v)
        sys.exit(1)

    print(f"Compliance check passed. Checked {checked} files.")

if __name__ == "__main__":
    check_pii()
