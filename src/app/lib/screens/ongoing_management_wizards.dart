import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';

// --- Fix This Wizard ---
class FixThisWizardScreen extends ConsumerStatefulWidget {
  final String agentId;
  const FixThisWizardScreen({super.key, required this.agentId});

  @override
  ConsumerState<FixThisWizardScreen> createState() => _FixThisWizardScreenState();
}

class _FixThisWizardScreenState extends ConsumerState<FixThisWizardScreen> {
  int _step = 0;
  bool _isApplying = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Agent Diagnostics')),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text('Help me fix this', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                    const SizedBox(height: 24),
                    if (_step == 0) ...[
                      const Text('We detected a connection timeout with the primary database. The agent was unable to read the required state.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      FilledButton(
                        onPressed: () => setState(() => _step = 1),
                        child: const Text('View Suggested Fix'),
                      ),
                    ] else if (_step == 1) ...[
                      const Text('Suggested fix: Restart the agent process and clear local cache to reconnect.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      _isApplying
                          ? const Center(child: CircularProgressIndicator())
                          : FilledButton(
                              onPressed: () async {
                                setState(() => _isApplying = true);
                                await Future.delayed(const Duration(seconds: 2));
                                if (mounted) setState(() { _isApplying = false; _step = 2; });
                              },
                              child: const Text('Apply Fix'),
                            ),
                    ] else if (_step == 2) ...[
                      const Icon(Icons.check_circle, color: Colors.green, size: 64),
                      const SizedBox(height: 16),
                      const Text('Fix applied successfully! The agent is healthy again.', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      FilledButton(
                        onPressed: () => context.go('/agents'),
                        child: const Text('Return to Agents'),
                      ),
                    ],
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

// --- Upgrade Wizard ---
class UpgradeWizardScreen extends ConsumerStatefulWidget {
  const UpgradeWizardScreen({super.key});

  @override
  ConsumerState<UpgradeWizardScreen> createState() => _UpgradeWizardScreenState();
}

class _UpgradeWizardScreenState extends ConsumerState<UpgradeWizardScreen> {
  int _progress = 0;
  bool _isUpgrading = false;
  bool _done = false;

  void _startUpgrade() async {
    setState(() { _isUpgrading = true; });
    for (int i = 1; i <= 4; i++) {
      await Future.delayed(const Duration(milliseconds: 800));
      if (mounted) setState(() => _progress = i);
    }
    if (mounted) setState(() { _done = true; _isUpgrading = false; });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Platform Upgrade')),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text('Upgrade to v2.4 ✨', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    if (!_isUpgrading && !_done) ...[
                      const Text("What's new:\n• Improved agent reasoning\n• 2x faster orchestration\n• New observability metrics", style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      FilledButton(
                        onPressed: _startUpgrade,
                        child: const Text('Upgrade in 1 click'),
                      ),
                    ] else if (_isUpgrading) ...[
                      LinearProgressIndicator(value: _progress / 4),
                      const SizedBox(height: 16),
                      Text(
                        _progress == 1 ? 'Downloading...' :
                        _progress == 2 ? 'Applying migrations...' :
                        _progress == 3 ? 'Restarting services...' : 'Finalizing...',
                        textAlign: TextAlign.center,
                        style: const TextStyle(fontFamily: 'Inter'),
                      ),
                      const SizedBox(height: 24),
                      TextButton(onPressed: () {}, child: const Text('Rollback')),
                    ] else if (_done) ...[
                      const Icon(Icons.celebration, color: Colors.blueAccent, size: 64),
                      const SizedBox(height: 16),
                      const Text('Upgrade complete!', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 24),
                      FilledButton(
                        onPressed: () => context.go('/dashboard'),
                        child: const Text('Go to Dashboard'),
                      ),
                    ],
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

// --- Billing Wizard ---
class BillingWizardScreen extends ConsumerWidget {
  const BillingWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(title: const Text('Billing & Credits')),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text('How much does this cost?', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                    const SizedBox(height: 24),
                    Container(
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        color: Colors.blue.withValues(alpha: 0.1),
                        borderRadius: BorderRadius.circular(12),
                        border: Border.all(color: Colors.blue.withValues(alpha: 0.3)),
                      ),
                      child: Column(
                        children: const [
                          Text('Current Usage', style: TextStyle(fontFamily: 'Inter', fontSize: 14)),
                          SizedBox(height: 8),
                          Text('\$42.50', style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold)),
                          SizedBox(height: 8),
                          Text('Projected monthly cost: \$85.00', style: TextStyle(fontFamily: 'Inter', fontSize: 12, color: Colors.grey)),
                        ],
                      ),
                    ),
                    const SizedBox(height: 24),
                    const Text('Your current plan includes 1000 AI tasks per day. You have used 450 today.', style: TextStyle(fontFamily: 'Inter', fontSize: 14)),
                    const SizedBox(height: 24),
                    FilledButton(
                      onPressed: () {},
                      child: const Text('Add Credits'),
                    ),
                    const SizedBox(height: 12),
                    OutlinedButton(
                      onPressed: () {},
                      child: const Text('Switch Plan'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}


// --- Review Pending Actions Wizard ---
class PendingAction {
  final String id;
  final String agentName;
  final String actionDescription;
  final String riskLevel;

  const PendingAction({
    required this.id,
    required this.agentName,
    required this.actionDescription,
    required this.riskLevel,
  });

  factory PendingAction.fromJson(Map<String, dynamic> json) {
    return PendingAction(
      id: json['id'] as String? ?? json['task_id'] as String? ?? '',
      agentName: json['agent_name'] as String? ?? 'Unknown Agent',
      actionDescription: json['action_description'] as String? ?? 'No description',
      riskLevel: json['risk_level'] as String? ?? 'Medium',
    );
  }
}

class PendingActionsState {
  final List<PendingAction> actions;
  final bool isLoading;

  const PendingActionsState({
    this.actions = const [],
    this.isLoading = false,
  });

  PendingActionsState copyWith({
    List<PendingAction>? actions,
    bool? isLoading,
  }) {
    return PendingActionsState(
      actions: actions ?? this.actions,
      isLoading: isLoading ?? this.isLoading,
    );
  }
}

class PendingActionsNotifier extends Notifier<PendingActionsState> {
  @override
  PendingActionsState build() {
    _loadActions();
    return const PendingActionsState(isLoading: true);
  }

  Future<void> _loadActions() async {
    try {
      final apiService = ref.read(apiServiceProvider);
      if (apiService == null) return;
      final actions = await apiService.getPendingActions();
      state = state.copyWith(isLoading: false, actions: actions);
    } catch (e) {
      state = state.copyWith(isLoading: false);
    }
  }

  Future<void> approveAction(BuildContext context, String id) async {
    try {
      final apiService = ref.read(apiServiceProvider);
      if (apiService == null) return;
      await apiService.approvePendingAction(id);
      state = state.copyWith(actions: state.actions.where((a) => a.id != id).toList());
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Action approved and executed.')),
        );
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Failed to approve action: $e')),
        );
      }
    }
  }

  Future<void> rejectAction(BuildContext context, String id) async {
    try {
      final apiService = ref.read(apiServiceProvider);
      if (apiService == null) return;
      await apiService.rejectPendingAction(id);
      state = state.copyWith(actions: state.actions.where((a) => a.id != id).toList());
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Action rejected and discarded.')),
        );
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Failed to reject action: $e')),
        );
      }
    }
  }
}

final pendingActionsProvider = NotifierProvider<PendingActionsNotifier, PendingActionsState>(() {
  return PendingActionsNotifier();
});

class ReviewPendingActionsWizardScreen extends ConsumerWidget {
  const ReviewPendingActionsWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(pendingActionsProvider);
    final notifier = ref.read(pendingActionsProvider.notifier);

    return Scaffold(
      appBar: AppBar(title: const Text('Review Actions')),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text('Pending Approvals', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    const Text('Review high-risk actions proposed by your AI agents before they are executed.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                    const SizedBox(height: 24),
                    if (state.isLoading)
                      const Center(child: CircularProgressIndicator())
                    else if (state.actions.isEmpty)
                      const Center(child: Text('All caught up! No pending actions.', style: TextStyle(fontFamily: 'Inter', fontSize: 16, fontStyle: FontStyle.italic)))
                    else
                      ...state.actions.map((action) => _PendingActionCard(action: action, onApprove: () => notifier.approveAction(context, action.id), onReject: () => notifier.rejectAction(context, action.id))),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _PendingActionCard extends StatelessWidget {
  final PendingAction action;
  final VoidCallback onApprove;
  final VoidCallback onReject;

  const _PendingActionCard({
    required this.action,
    required this.onApprove,
    required this.onReject,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white.withValues(alpha: 0.05),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.warning_amber, color: Colors.orange[300], size: 20),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  action.agentName,
                  style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 14),
                ),
              ),
            ],
          ),
          const SizedBox(height: 8),
          Text(
            action.actionDescription,
            style: const TextStyle(fontFamily: 'Inter', fontSize: 14),
          ),
          const SizedBox(height: 16),
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              TextButton(
                onPressed: onReject,
                style: TextButton.styleFrom(foregroundColor: Colors.red[300]),
                child: const Text('Reject', style: TextStyle(fontFamily: 'Inter')),
              ),
              const SizedBox(width: 8),
              FilledButton(
                onPressed: onApprove,
                child: const Text('Approve', style: TextStyle(fontFamily: 'Inter')),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
