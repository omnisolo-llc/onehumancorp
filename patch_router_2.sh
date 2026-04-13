#!/bin/bash
sed -i '/GoRoute(path: '"'"'\/dashboard'"'"'/a \          GoRoute(path: '"'"'\/kairos_dashboard'"'"', builder: (context, state) => const KairosDashboardScreen()),' srcs/app/lib/router.dart
