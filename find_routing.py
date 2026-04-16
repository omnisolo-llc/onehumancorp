import os
import glob

# Search for where an agent is instantiated and passed default vs other providers
for root, _, files in os.walk("srcs/server"):
    for file in files:
        if file.endswith(".go"):
            path = os.path.join(root, file)
            with open(path, "r") as f:
                content = f.read()
                if "ProviderTypeBuiltin" in content or "BuiltinAgent" in content or "NewAgentServiceServer" in content:
                    print(path)
