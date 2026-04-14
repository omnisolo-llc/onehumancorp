import sys

with open("srcs/server/dashboard/handlers_mcp.go", "r") as f:
    content = f.read()

import_blob = '"github.com/onehumancorp/mono/srcs/server/tools/blobinspector"'
import_hybridfs = '"github.com/onehumancorp/mono/lib/integrations/hybridfsmcp"'

if import_blob in content and import_hybridfs not in content:
    content = content.replace(import_blob, import_blob + "\n\t" + import_hybridfs)
    with open("srcs/server/dashboard/handlers_mcp.go", "w") as f:
        f.write(content)
    print("Success")
else:
    print("Failed or already imported")
