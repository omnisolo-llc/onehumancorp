import re

with open('srcs/server/orchestration/service.go', 'r') as f:
    content = f.read()

# Check if AdvertiseCapabilities is there
if 'AdvertiseCapabilities' not in content:
    print("AdvertiseCapabilities not found in service.go")
