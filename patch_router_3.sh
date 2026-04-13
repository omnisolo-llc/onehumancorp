#!/bin/bash
sed -i '/builder: (context, state) => const DashboardScreen(),/a \          ),\n          GoRoute(\n            path: '"'"'\/kairos_dashboard'"'"',\n            builder: (context, state) => const KairosDashboardScreen(),' srcs/app/lib/router.dart
