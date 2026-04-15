import sys
import os

print("Running all flutter tests via cd apps/web && flutter test")
result = os.system("cd apps/web && flutter test")
if result != 0:
    print("Tests failed")
    sys.exit(1)
print("Tests passed")
