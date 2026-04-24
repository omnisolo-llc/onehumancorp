import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import 'package:ohc_app/screens/help_portal_screen.dart';
import 'package:ohc_app/screens/release_notes_screen.dart';
import 'package:ohc_app/screens/api_docs_screen.dart';

import 'package:ohc_app/screens/ongoing_management_wizards.dart';

import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
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
import 'package:ohc_app/screens/diagnostics_screen.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/screens/handoffs_screen.dart';
import 'package:ohc_app/screens/cost_dashboard_screen.dart';
import 'package:ohc_app/screens/scaling_screen.dart';
import 'package:ohc_app/screens/pipelines_screen.dart';
import 'package:ohc_app/screens/integrations_screen.dart';
import 'package:ohc_app/screens/user_management_screen.dart';
import 'package:ohc_app/screens/agent_hire_wizard_screen.dart';
import 'package:ohc_app/screens/prompt_tuning_wizard_screen.dart';
import 'package:ohc_app/screens/landing_screen.dart';
import 'package:ohc_app/screens/landing_page_experiments_screen.dart';
import 'package:ohc_app/screens/swarm_memory_screen.dart';
import 'package:ohc_app/screens/autodream_sync_walkthrough_screen.dart';
import 'package:ohc_app/screens/referrals_dashboard_screen.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';

import 'package:ohc_app/services/auth_service.dart';
import 'package:flutter/material.dart';

/// A [ChangeNotifier] that bridges Riverpod [authStateProvider] changes to
/// [GoRouter.refreshListenable], so the router re-evaluates its redirect
/// guards without being fully recreated.
class _GoRouterAuthNotifier extends ChangeNotifier {
  _GoRouterAuthNotifier(Ref ref) {
    ref.listen(authStateProvider, (_, __) => notifyListeners());
  }
}

final routerProvider = Provider<GoRouter>((ref) {

  String? safeRedirectTarget(String? location) {
    if (location == null || location.isEmpty) return null;

    final uri = Uri.tryParse(location);
    if (uri == null || uri.hasScheme || uri.hasAuthority) return null;

    final target = uri.toString();
    if (!target.startsWith('/') || target.startsWith('/login')) return null;
    return target;
  }

  return GoRouter(
    initialLocation: '/landing',
    refreshListenable: _GoRouterAuthNotifier(ref),
    redirect: (context, state) {
      final authState = ref.read(authStateProvider);
      // Don't redirect while auth state is still loading (avoids navigation
      // resets during login/logout transitions).
      if (authState.isLoading) return null;

      final isLoggedIn = authState.valueOrNull != null;
      final isLoginRoute = state.matchedLocation == '/login';
      final isLandingRoute = state.matchedLocation == '/landing';
      final redirectTarget = safeRedirectTarget(state.uri.queryParameters['redirect']);

      // Allow these public routes without authentication.
      if (!isLoggedIn && !isLoginRoute && !isLandingRoute) {
        final encodedTarget = Uri.encodeComponent(state.uri.toString());
        return '/login?redirect=$encodedTarget';
      }
      if (isLoggedIn && isLoginRoute) return redirectTarget ?? '/dashboard';
      return null;
    },
    routes: [
      GoRoute(path: '/landing', builder: (context, state) => const LandingScreen()),
      GoRoute(path: '/login', builder: (context, state) => const LoginScreen()),
      ShellRoute(
        builder: (context, state, child) => AppShell(child: child),
        routes: [
          // Business setup requires authentication — moved inside the shell.
          GoRoute(
            path: '/business_setup',
            builder: (context, state) => const BusinessSetupWizardScreen(),
          ),
          GoRoute(
            path: '/orchestration/tasks',
            builder: (context, state) => const TaskListScreen(),
          ),
          GoRoute(
            path: '/diagnostics',
            builder: (context, state) => const DiagnosticsScreen(),
          ),

          GoRoute(
            path: '/help',
            builder: (context, state) => const HelpPortalScreen(),
          ),
          GoRoute(
            path: '/release-notes',
            builder: (context, state) => const ReleaseNotesScreen(),
          ),
          GoRoute(
            path: '/api-docs',
            builder: (context, state) => const ApiDocsScreen(),
          ),
GoRoute(
            path: '/dashboard',
            builder: (context, state) => const DashboardScreen(),
          ),
          GoRoute(
            path: '/agents',
            builder: (context, state) => const AgentsScreen(),
          ),
          GoRoute(
            path: '/meetings',
            builder: (context, state) => const MeetingsScreen(),
          ),
          GoRoute(
            path: '/chat',
            builder: (context, state) => const ChatScreen(),
          ),
          GoRoute(
            path: '/channels',
            builder: (context, state) => const ChannelsScreen(),
          ),
          GoRoute(
            path: '/ai-config',
            builder: (context, state) => const AiConfigScreen(),
          ),
          GoRoute(
            path: '/skills',
            builder: (context, state) => const SkillsScreen(),
          ),
          GoRoute(
            path: '/logs',
            builder: (context, state) => const LogsScreen(),
          ),
          GoRoute(
            path: '/security',
            builder: (context, state) => const SecurityScreen(),
          ),
          GoRoute(
            path: '/settings',
            builder: (context, state) => const SettingsScreen(),
          ),
          GoRoute(
            path: '/service',
            builder: (context, state) => const ServiceScreen(),
          ),
          GoRoute(
            path: '/wizard',
            builder: (context, state) => const SetupWizardScreen(),
          ),
          GoRoute(
            path: '/handoffs',
            builder: (context, state) => const HandoffsScreen(),
          ),
          GoRoute(
            path: '/cost',
            builder: (context, state) => const CostDashboardScreen(),
          ),
          GoRoute(
            path: '/scaling',
            builder: (context, state) => const ScalingScreen(),
          ),
          GoRoute(
            path: '/pipelines',
            builder: (context, state) => const PipelinesScreen(),
          ),
          GoRoute(
            path: '/integrations',
            builder: (context, state) => const IntegrationsScreen(),
          ),
          GoRoute(
            path: '/users',
            builder: (context, state) => const UserManagementScreen(),
          ),
          GoRoute(
            path: '/wizards/fix/:id',
            builder: (context, state) => FixThisWizardScreen(
              agentId: state.pathParameters['id'] ?? 'unknown',
            ),
          ),
          GoRoute(
            path: '/wizards/upgrade',
            builder: (context, state) => const UpgradeWizardScreen(),
          ),
          GoRoute(
            path: '/wizards/billing',
            builder: (context, state) => const BillingWizardScreen(),
          ),
          GoRoute(
            path: '/agents/:id/tune',
            builder: (context, state) => PromptTuningWizardScreen(
              agentId: state.pathParameters['id'] ?? 'unknown',
            ),
          ),
          GoRoute(
            path: '/agents/hire',
            builder: (context, state) => const AgentHireWizardScreen(),
          ),
          GoRoute(
            path: '/swarm-memory',
            builder: (context, state) => const SwarmMemoryScreen(),
          ),
          GoRoute(
            path: '/autodream-sync',
            builder: (context, state) => const AutoDreamSyncWalkthroughScreen(),
          ),
          GoRoute(
            path: '/growth-experiments',
            builder: (context, state) => const LandingPageExperimentsScreen(),
          ),
          GoRoute(
            path: '/referrals',
            builder: (context, state) => const ReferralsDashboardScreen(),
          ),
        ],
      ),
    ],
  );
});

/// Persistent shell (sidebar + navigation) wrapping all authenticated routes.

class AppShell extends StatefulWidget {
  final Widget child;
  const AppShell({super.key, required this.child});

  @override
  State<AppShell> createState() => _AppShellState();
}

class _AppShellState extends State<AppShell> {
  bool _isChatOpen = false;

  void _toggleChat() {
    setState(() {
      _isChatOpen = !_isChatOpen;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          Row(children: [_Sidebar(), Expanded(child: widget.child)]),
          if (_isChatOpen)
            Positioned(
              right: 24,
              bottom: 80,
              width: 350,
              height: 500,
              child: Material(
                color: Colors.transparent,
                child: Container(
                  decoration: BoxDecoration(
                    color: const Color(0xFF1E293B),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: Colors.white24),
                    boxShadow: [
                      BoxShadow(color: Colors.black54, blurRadius: 10, offset: Offset(0, 4))
                    ]
                  ),
                  child: Column(
                    children: [
                      Container(
                        padding: const EdgeInsets.all(16),
                        decoration: const BoxDecoration(
                          color: Color(0xFF0F172A),
                          borderRadius: BorderRadius.only(topLeft: Radius.circular(12), topRight: Radius.circular(12)),
                        ),
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            const Text('AI Help Chat', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                            IconButton(
                              icon: const Icon(Icons.close, color: Colors.white),
                              onPressed: _toggleChat,
                              padding: EdgeInsets.zero,
                              constraints: const BoxConstraints(),
                            )
                          ],
                        ),
                      ),
                      Expanded(
                        child: ListView(
                          padding: const EdgeInsets.all(16),
                          children: const [
                            Text('Hi there! I am your AI Help Agent. How can I assist you with One Human Corp today?', style: TextStyle(color: Colors.white70)),
                          ],
                        )
                      ),
                      Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: TextField(
                          style: const TextStyle(color: Colors.white),
                          decoration: InputDecoration(
                            hintText: 'Ask anything...',
                            hintStyle: const TextStyle(color: Colors.white54),
                            filled: true,
                            fillColor: Colors.black26,
                            border: OutlineInputBorder(borderRadius: BorderRadius.circular(20), borderSide: BorderSide.none),
                            contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                            suffixIcon: const Icon(Icons.send, color: Colors.blueAccent),
                          ),
                        ),
                      )
                    ],
                  ),
                ),
              ),
            ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _toggleChat,
        backgroundColor: Colors.blueAccent,
        child: const Icon(Icons.support_agent),
      ),
    );
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
        _NavItem(icon: Icons.smart_toy, label: 'Agents', path: '/agents'),
        _NavItem(
          icon: Icons.checklist,
          label: 'Shared Tasks',
          path: '/orchestration/tasks',
        ),
        _NavItem(icon: Icons.memory, label: 'Swarm Memory', path: '/swarm-memory'),
        _NavItem(icon: Icons.sync, label: 'AutoDream Sync', path: '/autodream-sync'),
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

        const Divider(),
        _NavItem(icon: Icons.help_outline, label: 'Help Portal', path: '/help'),
        _NavItem(icon: Icons.new_releases, label: 'Release Notes', path: '/release-notes'),
        _NavItem(icon: Icons.code, label: 'API Docs', path: '/api-docs'),
const SizedBox(height: 16),
        _NavItem(
          icon: Icons.health_and_safety,
          label: 'Diagnostics',
          path: '/diagnostics',
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
