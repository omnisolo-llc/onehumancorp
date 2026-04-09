import re

filepath = "srcs/app/lib/screens/wizard_screen.dart"

with open(filepath, 'r') as f:
    content = f.read()

# Replace buildGlassCard declaration
content = re.sub(
    r'Widget buildGlassCard\(\{required Color color, required Widget child\}\) \{.*?return ClipRRect\(.*?child:\s*child,\s*\),\s*\),\s*\);\s*\}',
    '',
    content,
    flags=re.DOTALL
)

# Replace buildGlassCard calls
content = content.replace("buildGlassCard(", "GlassCard(")

if "import 'package:ohc_app/widgets/glass_card.dart';" not in content:
    content = "import 'package:ohc_app/widgets/glass_card.dart';\n" + content

with open(filepath, 'w') as f:
    f.write(content)
