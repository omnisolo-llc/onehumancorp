with open("srcs/app/lib/services/api_service.dart", "r") as f:
    content = f.read()

import re

# Fix _headers getter
content = re.sub(
    r"  Map<String, String> get _headers => \{\n    _headers,\n    'Authorization': 'Bearer \$token',\n  \};",
    "  Map<String, String> get _headers => {\n    'Content-Type': 'application/json',\n    'Authorization': 'Bearer $token',\n  };",
    content
)

# Fix createReferral
content = re.sub(
    r"      headers: \{\n        _headers,\n        'Authorization': 'Bearer \$token',\n      \},",
    "      headers: {\n        'Content-Type': 'application/json',\n        'Authorization': 'Bearer $token',\n      },",
    content
)

with open("srcs/app/lib/services/api_service.dart", "w") as f:
    f.write(content)
