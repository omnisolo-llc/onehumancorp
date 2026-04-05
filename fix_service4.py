with open("srcs/server/orchestration/service.go", "r") as f:
    service = f.read()

import re

# find import block
imports = re.search(r'import \((.*?)\)', service, re.DOTALL)
if imports:
    imp = imports.group(1)
    if '"github.com/onehumancorp/mono/srcs/server/auth"' not in imp:
        new_imp = imp + '\t"github.com/onehumancorp/mono/srcs/server/auth"\n'
        service = service.replace(imp, new_imp)
        with open("srcs/server/orchestration/service.go", "w") as f:
            f.write(service)
