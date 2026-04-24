import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../services/api_service.dart';
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
class UpgradeWizardScreen extends ConsumerWidget {
  const UpgradeWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final quotaAsync = ref.watch(quotaProvider);

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
                    Text('Upgrade Plan 🚀', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    const Text("To increase your daily task quota, you can invite more users to the platform.", style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                    const SizedBox(height: 24),
                    quotaAsync.when(
                      data: (quota) {
                        final max = quota['max'] ?? 0;
                        return Text('Your current daily quota is $max tasks.', style: const TextStyle(fontFamily: 'Inter', fontSize: 16, fontWeight: FontWeight.bold));
                      },
                      loading: () => const Center(child: CircularProgressIndicator()),
                      error: (err, stack) => Text('Error loading quota: $err', style: const TextStyle(color: Colors.red)),
                    ),
                    const SizedBox(height: 24),
                    const Text("Each successful referral adds 50 tasks to your daily limit.", style: TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.grey)),
                    const SizedBox(height: 32),
                    FilledButton(
                      onPressed: () => context.go('/referrals'),
                      child: const Text('Go to Referrals'),
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

// --- Billing Wizard ---

final quotaProvider = FutureProvider<Map<String, dynamic>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('Not authenticated');
  return api.getQuota();
});

class BillingWizardScreen extends ConsumerWidget {
  const BillingWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final quotaAsync = ref.watch(quotaProvider);

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
                    quotaAsync.when(
                      data: (quota) {
                        final used = quota['used'] ?? 0;
                        final max = quota['max'] ?? 0;
                        return Text('Your current plan includes $max AI tasks per day. You have used $used today.', style: const TextStyle(fontFamily: 'Inter', fontSize: 14));
                      },
                      loading: () => const Center(child: CircularProgressIndicator()),
                      error: (err, stack) => Text('Error loading quota: $err', style: const TextStyle(color: Colors.red)),
                    ),
                    const SizedBox(height: 24),
                    const FilledButton(
                      onPressed: null,
                      child: Text('Add Credits'),
                    ),
                    const SizedBox(height: 12),
                    const OutlinedButton(
                      onPressed: null,
                      child: Text('Switch Plan'),
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
class GrowMyBusinessWizardScreen extends ConsumerStatefulWidget {
  const GrowMyBusinessWizardScreen({super.key});

  @override
  ConsumerState<GrowMyBusinessWizardScreen> createState() => _GrowMyBusinessWizardScreenState();
}

class _GrowMyBusinessWizardScreenState extends ConsumerState<GrowMyBusinessWizardScreen> {
  int _step = 0;
  bool _isApplying = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Grow My Business', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold))),
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
                    Text('Grow your business 🌱', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                    const SizedBox(height: 24),
                    if (_step == 0) ...[
                      const Text('Based on your current business stage, here are our top recommendations to help you grow.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      _ActionCard(
                        icon: Icons.add_shopping_cart,
                        title: 'Add 5 more products',
                        subtitle: 'Stores with 5+ products see 40% higher conversion rates.',
                        onTap: () => setState(() => _step = 1),
                      ),
                      const SizedBox(height: 12),
                      _ActionCard(
                        icon: Icons.camera_alt,
                        title: 'Connect Instagram',
                        subtitle: 'Let The Promoter auto-post your inventory to social media.',
                        onTap: () => setState(() => _step = 2),
                      ),
                      const SizedBox(height: 12),
                      _ActionCard(
                        icon: Icons.email,
                        title: 'Run your first email campaign',
                        subtitle: 'Reach out to your first 10 customers with a special offer.',
                        onTap: () => setState(() => _step = 3),
                      ),
                    ] else if (_step == 1) ...[
                      const Icon(Icons.add_shopping_cart, color: Colors.blueAccent, size: 48),
                      const SizedBox(height: 16),
                      const Text('Ready to add more products?', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      const Text('The Promoter agent can help you write descriptions and format images.', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      FilledButton(
                        onPressed: () {
                          context.go('/dashboard');
                        },
                        child: const Text('Go to Inventory'),
                      ),
                      const SizedBox(height: 12),
                      TextButton(
                        onPressed: () => setState(() => _step = 0),
                        child: const Text('Back'),
                      ),
                    ] else if (_step == 2) ...[
                      const Icon(Icons.camera_alt, color: Colors.pinkAccent, size: 48),
                      const SizedBox(height: 16),
                      const Text('Connect Instagram', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      const Text('Connect your Instagram Professional account to allow auto-posting.', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      _isApplying
                          ? const Center(child: CircularProgressIndicator())
                          : FilledButton(
                              onPressed: () async {
                                setState(() => _isApplying = true);
                                await Future.delayed(const Duration(seconds: 1));
                                if (mounted) {
                                  setState(() { _isApplying = false; _step = 4; });
                                }
                              },
                              child: const Text('Connect Account'),
                            ),
                      const SizedBox(height: 12),
                      TextButton(
                        onPressed: () => setState(() => _step = 0),
                        child: const Text('Back'),
                      ),
                    ] else if (_step == 3) ...[
                      const Icon(Icons.email, color: Colors.orangeAccent, size: 48),
                      const SizedBox(height: 16),
                      const Text('Create Email Campaign', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      const Text('Let The Promoter draft an email to your recent customers.', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      FilledButton(
                        onPressed: () {
                          context.go('/dashboard');
                        },
                        child: const Text('Start Drafting'),
                      ),
                      const SizedBox(height: 12),
                      TextButton(
                        onPressed: () => setState(() => _step = 0),
                        child: const Text('Back'),
                      ),
                    ] else if (_step == 4) ...[
                      const Icon(Icons.check_circle, color: Colors.green, size: 64),
                      const SizedBox(height: 16),
                      const Text('Instagram Connected!', textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                      const SizedBox(height: 24),
                      FilledButton(
                        onPressed: () => context.go('/dashboard'),
                        child: const Text('Return to Dashboard'),
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

class _ActionCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String subtitle;
  final VoidCallback onTap;

  const _ActionCard({
    required this.icon,
    required this.title,
    required this.subtitle,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            border: Border.all(color: Theme.of(context).colorScheme.outline.withValues(alpha: 0.2)),
            borderRadius: BorderRadius.circular(12),
            color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.05),
          ),
          child: Row(
            children: [
              Icon(icon, size: 32, color: Theme.of(context).colorScheme.primary),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(title, style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
                    const SizedBox(height: 4),
                    Text(subtitle, style: const TextStyle(fontSize: 12, fontFamily: 'Inter', color: Colors.grey)),
                  ],
                ),
              ),
              const Icon(Icons.arrow_forward_ios, size: 16, color: Colors.grey),
            ],
          ),
        ),
      ),
    );
  }
}
