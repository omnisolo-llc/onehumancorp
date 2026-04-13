#!/bin/bash
sed -i 's/import '\''package:ohc_app\/screens\/wizard_screen.dart'\'';/import '\''package:ohc_app\/screens\/wizard_screen.dart'\'';\nimport '\''package:ohc_app\/screens\/business_setup_wizard_screen.dart'\'';/' srcs/app/lib/router.dart
sed -i 's/          GoRoute(/          GoRoute(\n            path: '\''\/business_setup'\'',\n            builder: (context, state) => const BusinessSetupWizardScreen(),\n          ),\n          GoRoute(/' srcs/app/lib/router.dart
