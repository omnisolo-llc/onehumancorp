import time, sys
for _ in range(60):
    try:
        with open("bazel_test.log", "r") as f:
            content = f.read()
            if "Executed" in content or "FAILED" in content or "Build completed" in content:
                print("Tests finished.")
                sys.exit(0)
    except FileNotFoundError:
        pass
    time.sleep(5)
print("Timeout waiting for tests.")
sys.exit(1)
