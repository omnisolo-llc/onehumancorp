#!/bin/bash
sed -i '/import '"'"'package:ohc_app\/screens\/dashboard_screen.dart'"'"';/a import '"'"'package:ohc_app\/screens\/kairos_dashboard.dart'"'"';' srcs/app/lib/router.dart
sed -i '/GoRoute(path: '"'"'\/dashboard'"'"', builder: (context, state) => const DashboardScreen()),/a \          GoRoute(path: '"'"'\/kairos_dashboard'"'"', builder: (context, state) => const KairosDashboardScreen()),' srcs/app/lib/router.dart
