import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';
import 'dart:ui';

class ReferralLeaderboardScreen extends ConsumerStatefulWidget {
  const ReferralLeaderboardScreen({super.key});

  @override
  ConsumerState<ReferralLeaderboardScreen> createState() =>
      _ReferralLeaderboardScreenState();
}

class _ReferralLeaderboardScreenState extends ConsumerState<ReferralLeaderboardScreen> {
  late Future<List<Map<String, dynamic>>> _leaderboardFuture;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _leaderboardFuture = ref.read(apiServiceProvider)!.getReferralLeaderboard();
    });
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Growth Referral Leaderboard'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _refresh,
          ),
        ],
      ),
      body: FutureBuilder<List<Map<String, dynamic>>>(
        future: _leaderboardFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          }
          if (snapshot.hasError) {
            return Center(
              child: Text(
                'Error: ${snapshot.error}',
                style: TextStyle(color: colors.error),
              ),
            );
          }

          final leaderboard = snapshot.data ?? [];

          if (leaderboard.isEmpty) {
            return const Center(
              child: Text(
                'No referrals tracked yet.',
                style: TextStyle(fontFamily: 'Inter', fontSize: 16),
              ),
            );
          }

          return ListView.separated(
            padding: const EdgeInsets.all(24),
            itemCount: leaderboard.length,
            separatorBuilder: (context, index) => const SizedBox(height: 12),
            itemBuilder: (context, index) {
              final entry = leaderboard[index];
              return _LeaderboardCard(
                entry: entry,
                rank: index + 1,
              );
            },
          );
        },
      ),
    );
  }
}

class _LeaderboardCard extends StatelessWidget {
  final Map<String, dynamic> entry;
  final int rank;

  const _LeaderboardCard({required this.entry, required this.rank});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    // Give top 3 special visual styling
    final bool isTopThree = rank <= 3;
    final Color rankColor = rank == 1 ? Colors.amber : rank == 2 ? Colors.grey[300]! : rank == 3 ? Colors.brown[300]! : colors.onSurfaceVariant;

    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(20),
          decoration: BoxDecoration(
            color: isTopThree ? colors.primary.withValues(alpha: 0.1) : colors.surface.withValues(alpha: 0.6),
            border: Border.all(
              color: isTopThree ? colors.primary.withValues(alpha: 0.3) : colors.onSurface.withValues(alpha: 0.1),
              width: 1.5,
            ),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Row(
            children: [
              Container(
                width: 40,
                height: 40,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: isTopThree ? rankColor.withValues(alpha: 0.2) : colors.surface,
                  border: Border.all(color: rankColor, width: 2),
                ),
                alignment: Alignment.center,
                child: Text(
                  '#$rank',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.bold,
                    color: rankColor,
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      entry['userId'] ?? 'Unknown User',
                      style: const TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      'Sovereign Growth Champion',
                      style: TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 12,
                        color: colors.onSurfaceVariant,
                      ),
                    ),
                  ],
                ),
              ),
              Column(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  Text(
                    '${entry['conversions']} Conversions',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                      color: colors.primary,
                    ),
                  ),
                  Text(
                    '${entry['totalClicks']} Clicks',
                    style: TextStyle(
                      fontFamily: 'Inter',
                      fontSize: 12,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
