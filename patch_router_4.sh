#!/bin/bash
sed -i '/_NavItem(icon: Icons.dashboard, label: '"'"'Dashboard'"'"', path: '"'"'\/dashboard'"'"'),/a \        _NavItem(icon: Icons.analytics, label: '"'"'KAIROS Analytics'"'"', path: '"'"'\/kairos_dashboard'"'"'),' srcs/app/lib/router.dart
