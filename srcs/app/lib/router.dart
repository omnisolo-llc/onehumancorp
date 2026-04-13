import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:app/screens/kairos_dashboard.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:app/screens/kairos_dashboard.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:ohc_app/screens/agents_screen.dart';
import 'package:ohc_app/screens/meetings_screen.dart';
import 'package:ohc_app/screens/chat_screen.dart';
import 'package:ohc_app/screens/channels_screen.dart';
import 'package:ohc_app/screens/ai_config_screen.dart';
import 'package:ohc_app/screens/skills_screen.dart';
import 'package:ohc_app/screens/logs_screen.dart';
import 'package:ohc_app/screens/security_screen.dart';
import 'package:ohc_app/screens/settings_screen.dart';
import 'package:ohc_app/screens/service_screen.dart';
import 'package:ohc_app/screens/wizard_screen.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/screens/handoffs_screen.dart';
import 'package:ohc_app/screens/cost_dashboard_screen.dart';
import 'package:ohc_app/screens/scaling_screen.dart';
import 'package:ohc_app/screens/pipelines_screen.dart';
import 'package:ohc_app/screens/integrations_screen.dart';
import 'package:ohc_app/screens/user_management_screen.dart';
import 'package:ohc_app/screens/agent_hire_wizard_screen.dart';
import 'package:ohc_app/screens/landing_screen.dart';
import 'package:ohc_app/screens/landing_page_experiments_screen.dart';
import 'package:ohc_app/screens/swarm_memory_screen.dart';
import 'package:ohc_app/screens/referrals_dashboard_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:flutter/material.dart';

final routerProvider = Provider<GoRouter>((ref) {
  final authState = ref.watch(authStateProvider);

  return GoRouter(
    initialLocation: '/landing',
    redirect: (context, state) {
      final isLoggedIn = authState.valueOrNull != null;
      final isLoginRoute = state.matchedLocation == '/login';
      final isLandingRoute = state.matchedLocation == '/landing';

      if (!isLoggedIn && !isLoginRoute && !isLandingRoute) return '/landing';
      if (isLoggedIn && isLoginRoute) return '/dashboard';
      return null;
    },
    routes: [
      GoRoute(path: '/landing', builder: (context, state) => const LandingScreen()),
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
      GoRoute(path: '/login', builder: (context, state) => const LoginScreen()),
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
      ShellRoute(
        builder: (context, state, child) => AppShell(child: child),
        routes: [
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/business_setup',
            builder: (context, state) => const BusinessSetupWizardScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/dashboard',
            builder: (context, state) => const DashboardScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/agents',
            builder: (context, state) => const AgentsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/meetings',
            builder: (context, state) => const MeetingsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/chat',
            builder: (context, state) => const ChatScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
          GoRoute(
            path: '/kairos',
            builder: (context, state) => KairosDashboardScreen(
              channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
            ),
          ),
            path: '/channels',
            builder: (context, state) => const ChannelsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/ai-config',
            builder: (context, state) => const AiConfigScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/skills',
            builder: (context, state) => const SkillsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/logs',
            builder: (context, state) => const LogsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/security',
            builder: (context, state) => const SecurityScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/settings',
            builder: (context, state) => const SettingsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/service',
            builder: (context, state) => const ServiceScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/wizard',
            builder: (context, state) => const SetupWizardScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/handoffs',
            builder: (context, state) => const HandoffsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/cost',
            builder: (context, state) => const CostDashboardScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/scaling',
            builder: (context, state) => const ScalingScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/pipelines',
            builder: (context, state) => const PipelinesScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/integrations',
            builder: (context, state) => const IntegrationsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/users',
            builder: (context, state) => const UserManagementScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/agents/hire',
            builder: (context, state) => const AgentHireWizardScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/swarm-memory',
            builder: (context, state) => const SwarmMemoryScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/growth-experiments',
            builder: (context, state) => const LandingPageExperimentsScreen(),
          ),
          GoRoute(
        GoRoute(
          path: '/kairos',
          builder: (context, state) => KairosDashboardScreen(
            channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
          ),
        ),
            path: '/referrals',
            builder: (context, state) => const ReferralsDashboardScreen(),
          ),
        ],
      ),
    ],
  );
});

/// Persistent shell (sidebar + navigation) wrapping all authenticated routes.
class AppShell extends StatelessWidget {
  final Widget child;
  const AppShell({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: Row(children: [_Sidebar(), Expanded(child: child)]));
  }
}

class _Sidebar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return NavigationDrawer(
      children: [
        const SizedBox(height: 16),
        const Padding(
          padding: EdgeInsets.symmetric(horizontal: 16),
          child: Text(
            'One Human Corp',
            style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
          ),
        ),
        const Divider(),
        _NavItem(icon: Icons.dashboard, label: 'Dashboard', path: '/dashboard'),
        _NavItem(icon: Icons.auto_graph, label: 'Swarm Analytics', path: '/kairos'),
        _NavItem(icon: Icons.smart_toy, label: 'Agents', path: '/agents'),
        _NavItem(icon: Icons.memory, label: 'Swarm Memory', path: '/swarm-memory'),
        _NavItem(icon: Icons.video_call, label: 'Meetings', path: '/meetings'),
        _NavItem(icon: Icons.chat, label: 'Chat', path: '/chat'),
        _NavItem(
          icon: Icons.transfer_within_a_station,
          label: 'Handoffs',
          path: '/handoffs',
        ),
        _NavItem(icon: Icons.bar_chart, label: 'Cost & Usage', path: '/cost'),
        _NavItem(
          icon: Icons.rocket_launch,
          label: 'Dynamic Scaling',
          path: '/scaling',
        ),
        _NavItem(icon: Icons.alt_route, label: 'Pipelines', path: '/pipelines'),
        _NavItem(
          icon: Icons.science,
          label: 'Growth Experiments',
          path: '/growth-experiments',
        ),
        _NavItem(
          icon: Icons.group_add,
          label: 'Viral Referrals',
          path: '/referrals',
        ),
        _NavItem(
          icon: Icons.extension,
          label: 'Integrations & Tools',
          path: '/integrations',
        ),
        _NavItem(
          icon: Icons.people_outline,
          label: 'User Management',
          path: '/users',
        ),
        _NavItem(
          icon: Icons.chat_bubble_outline,
          label: 'Channels',
          GoRoute(
            path: '/kairos',
            builder: (context, state) => KairosDashboardScreen(
              channel: WebSocketChannel.connect(Uri.parse('ws://localhost:3000/api/kairos/stream')),
            ),
          ),
          path: '/channels',
        ),
        const Divider(),
        _NavItem(
          icon: Icons.psychology,
          label: 'AI Providers',
          path: '/ai-config',
        ),
        _NavItem(icon: Icons.extension, label: 'Skills', path: '/skills'),
        _NavItem(icon: Icons.security, label: 'Security', path: '/security'),
        _NavItem(icon: Icons.terminal, label: 'Logs', path: '/logs'),
        const SizedBox(height: 8),
        _NavItem(icon: Icons.settings, label: 'Settings', path: '/settings'),
        _NavItem(
          icon: Icons.computer,
          label: 'Service Management',
          path: '/service',
        ),
        _NavItem(
          icon: Icons.auto_fix_high,
          label: 'Setup Wizard',
          path: '/wizard',
        ),
        const SizedBox(height: 16),
      ],
    );
  }
}

class _NavItem extends StatelessWidget {
  final IconData icon;
  final String label;
  final String path;

  const _NavItem({required this.icon, required this.label, required this.path});

  @override
  Widget build(BuildContext context) {
    final current = GoRouterState.of(context).matchedLocation;
    final selected = current.startsWith(path);
    return ListTile(
      leading: Icon(icon),
      title: Text(label),
      selected: selected,
      onTap: () => context.go(path),
    );
  }
}
