import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/approval.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

final approvalsProvider = FutureProvider.autoDispose<List<ApprovalRequest>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getApprovals();
});

class AgentActivityFeedWidget extends ConsumerWidget {
  const AgentActivityFeedWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final approvalsAsyncValue = ref.watch(approvalsProvider);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Agent Actions Today',
          style: Theme.of(context).textTheme.headlineSmall?.copyWith(
            fontWeight: FontWeight.bold,
            fontFamily: 'Outfit',
          ),
        ),
        const SizedBox(height: 16),
        approvalsAsyncValue.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (err, stack) => Text('Error: $err'),
          data: (approvals) {
            final pendingApprovals = approvals.where((a) => a.status == 'PENDING').toList();
            if (pendingApprovals.isEmpty) {
              return const Text('No pending actions.', style: TextStyle(fontFamily: 'Inter'));
            }
            return Column(
              children: pendingApprovals.map((approval) {
                return Padding(
                  padding: const EdgeInsets.only(bottom: 8.0),
                  child: GlassCard(
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            approval.action,
                            style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                          ),
                          const SizedBox(height: 4),
                          Text(approval.reason, style: const TextStyle(fontFamily: 'Inter')),
                          const SizedBox(height: 12),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.end,
                            children: [
                              TextButton(
                                onPressed: () async {
                                  final api = ref.read(apiServiceProvider);
                                  await api?.decideApproval(approval.id, 'reject');
                                  ref.invalidate(approvalsProvider);
                                },
                                child: const Text('Edit / Reject', style: TextStyle(fontFamily: 'Outfit')),
                              ),
                              const SizedBox(width: 8),
                              ElevatedButton(
                                onPressed: () async {
                                  final api = ref.read(apiServiceProvider);
                                  await api?.decideApproval(approval.id, 'approve');
                                  ref.invalidate(approvalsProvider);
                                },
                                child: const Text('Approve & Send', style: TextStyle(fontFamily: 'Outfit')),
                              ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),
                );
              }).toList(),
            );
          },
        ),
      ],
    );
  }
}
