import 'package:flutter/material.dart';import 'package:ohc_app/widgets/glass_card.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';
import 'dart:ui';
import 'package:intl/intl.dart';

class ReferralsDashboardScreen extends ConsumerStatefulWidget {
  const ReferralsDashboardScreen({super.key});

  @override
  ConsumerState<ReferralsDashboardScreen> createState() =>
      _ReferralsDashboardScreenState();
}

class _ReferralsDashboardScreenState extends ConsumerState<ReferralsDashboardScreen> {
  late Future<Map<String, dynamic>> _viralCoefficientFuture;
  late Future<List<Map<String, dynamic>>> _referralsFuture;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

void _refresh() {
    setState(() {
      _referralsFuture = ref.read(apiServiceProvider)!.listReferrals();
      _viralCoefficientFuture = ref.read(apiServiceProvider)!.getViralCoefficient();
    });
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Viral Loop Dashboard'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _refresh,
          ),
        ],
      ),
      body: FutureBuilder<Map<String, dynamic>>(
        future: _viralCoefficientFuture,
        builder: (context, viralSnapshot) {
          if (viralSnapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          }
          if (viralSnapshot.hasError) {
            return Center(
              child: Text(
                'Error: ${viralSnapshot.error}',
                style: TextStyle(color: colors.error),
              ),
            );
          }

          final viralData = viralSnapshot.data ?? {};
          final kFactor = viralData['kFactor'] ?? 0.0;
          final totalConversions = viralData['totalConversions'] ?? 0;
          final uniqueInviters = viralData['uniqueInviters'] ?? 0;

          return FutureBuilder<List<Map<String, dynamic>>>(
            future: _referralsFuture,
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

              final referrals = snapshot.data ?? [];

              return SingleChildScrollView(
                padding: const EdgeInsets.all(24),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    _KFactorCard(
                      kFactor: kFactor,
                      totalConversions: totalConversions,
                      uniqueInviters: uniqueInviters,
                    ),
                    const SizedBox(height: 32),
                    if (referrals.isEmpty)
                      const Center(
                        child: Text(
                          'No referrals tracked yet.',
                          style: TextStyle(fontFamily: 'Inter', fontSize: 16),
                        ),
                      )
                    else
                      Wrap(
              spacing: 24,
              runSpacing: 24,
              children: referrals.map((r) {
                return _ReferralCard(referral: r);
              }).toList(),
                      ),
                  ],
                ),
              );
            },
          );
        },
      ),
    );
  }
}

class _ReferralCard extends StatelessWidget {
  final Map<String, dynamic> referral;

  const _ReferralCard({required this.referral});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final createdAt = DateTime.tryParse(referral['createdAt'] ?? '') ?? DateTime.now();

    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 400),
      child: GlassCard(
        padding: const EdgeInsets.all(24),
        color: colors.surface.withValues(alpha: 0.6),
        child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      'Ref: ${referral['referralCode']}',
                      style: const TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    Icon(
                      Icons.group_add,
                      color: colors.primary,
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                Text(
                  'User: ${referral['userId']}',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: colors.onSurfaceVariant,
                  ),
                ),
                const SizedBox(height: 16),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceAround,
                  children: [
                    _StatColumn(label: 'Clicks', value: '${referral['clicks']}'),
                    _StatColumn(label: 'Conversions', value: '${referral['conversions']}'),
                  ],
                ),
                const SizedBox(height: 16),
                Text(
                  'Created: ${DateFormat.yMMMd().add_jm().format(createdAt)}',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 12,
                    color: colors.onSurfaceVariant,
                  ),
                ),
              ],
            ),
      ),
    );
  }
}

class _StatColumn extends StatelessWidget {
  final String label;
  final String value;

  const _StatColumn({required this.label, required this.value});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Column(
      children: [
        Text(
          value,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 24,
            fontWeight: FontWeight.bold,
            color: colors.primary,
          ),
        ),
        Text(
          label,
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: 12,
            color: colors.onSurfaceVariant,
          ),
        ),
      ],
    );
  }
}

class _KFactorCard extends StatelessWidget {
  final double kFactor;
  final int totalConversions;
  final int uniqueInviters;

  const _KFactorCard({
    required this.kFactor,
    required this.totalConversions,
    required this.uniqueInviters,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return GlassCard(
      padding: const EdgeInsets.all(32),
      color: colors.primaryContainer.withValues(alpha: 0.8),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceAround,
        children: [
          _StatColumn(label: 'K-Factor', value: kFactor.toStringAsFixed(2)),
          _StatColumn(label: 'Total Conversions', value: '$totalConversions'),
          _StatColumn(label: 'Unique Inviters', value: '$uniqueInviters'),
        ],
      ),
    );
  }
}
