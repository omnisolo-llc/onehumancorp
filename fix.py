import re

files = [
    'srcs/app/lib/screens/landing_screen.dart',
    'srcs/app/lib/screens/referrals_dashboard_screen.dart',
    'srcs/app/lib/screens/swarm_memory_screen.dart',
    'srcs/app/lib/screens/wizard_screen.dart',
]

for file_path in files:
    with open(file_path, 'r') as f:
        content = f.read()

    # Remove the duplicate import
    content = content.replace("import '../widgets/glass_card.dart';import 'package:ohc_app/widgets/glass_card.dart';", "import 'package:ohc_app/widgets/glass_card.dart';")
    content = content.replace("import 'dart:ui';\nimport 'package:ohc_app/widgets/glass_card.dart';", "import 'dart:ui';\nimport 'package:ohc_app/widgets/glass_card.dart';\n")
    content = content.replace("import 'package:intl/intl.dart';\nimport 'package:ohc_app/widgets/glass_card.dart';", "import 'package:intl/intl.dart';\nimport 'package:ohc_app/widgets/glass_card.dart';\n")
    content = content.replace("import 'package:ohc_app/widgets/glass_card.dart';\nimport 'package:ohc_app/widgets/glass_card.dart';", "import 'package:ohc_app/widgets/glass_card.dart';")

    with open(file_path, 'w') as f:
        f.write(content)
