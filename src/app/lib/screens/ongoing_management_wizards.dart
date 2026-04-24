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
