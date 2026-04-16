with open("srcs/server/telemetry/telemetry.go", "r") as f:
    lines = f.readlines()

for idx, line in enumerate(lines):
    if "payloadMap :=" in line:
        pass
