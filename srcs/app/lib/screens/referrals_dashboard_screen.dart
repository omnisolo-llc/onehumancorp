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
  late Future<List<Map<String, dynamic>>> _referralsFuture;
  late Future<Map<String, dynamic>> _viralCoefficientFuture;
  late Future<List<dynamic>> _combinedFuture;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _referralsFuture = ref.read(apiServiceProvider)!.listReferrals();
      _viralCoefficientFuture = ref.read(apiServiceProvider)!.getViralCoefficient();
      _combinedFuture = Future.wait([_referralsFuture, _viralCoefficientFuture]);
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
      body: FutureBuilder<List<dynamic>>(
        future: _combinedFuture,
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
          final referrals = snapshot.data?[0] as List<Map<String, dynamic>>? ?? [];
          final vcData = snapshot.data?[1] as Map<String, dynamic>? ?? {};
          final kFactor = vcData['kFactor'] ?? 0.0;
          return Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Padding(
                padding: const EdgeInsets.all(24),
                child: GlassCard(
                  padding: const EdgeInsets.all(24),
                  color: colors.surface.withValues(alpha: 0.6),
                  child: Column(
                    children: [
                      const Text(
                        'Viral Coefficient (K-Factor)',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 16),
                      Text(
                        kFactor.toStringAsFixed(2),
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 48,
                          fontWeight: FontWeight.bold,
                          color: colors.primary,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              Expanded(
                child: Builder(
              builder: (context) {
                if (referrals.isEmpty) {
            return const Center(
              child: Text(
                'No referrals tracked yet.',
                style: TextStyle(fontFamily: 'Inter', fontSize: 16),
              ),
            );
          }

          return SingleChildScrollView(
            padding: const EdgeInsets.all(24),
            child: Wrap(
              spacing: 24,
              runSpacing: 24,
              children: referrals.map((r) {
                return _ReferralCard(referral: r);
              }).toList(),
            ),
          );
              },
            ),
          ),
        ],
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
