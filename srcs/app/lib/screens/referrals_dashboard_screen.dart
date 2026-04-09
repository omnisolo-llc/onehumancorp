import 'package:flutter/material.dart';
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
  late Future<Map<String, dynamic>> _kFactorFuture;

  @override
  void initState() {
    super.initState();
    // Initialize both futures explicitly if _refresh relies on context read
    _referralsFuture = ref.read(apiServiceProvider)!.listReferrals();
    _kFactorFuture = ref.read(apiServiceProvider)!.getViralCoefficient();
  }

  void _refresh() {
    setState(() {
      _referralsFuture = ref.read(apiServiceProvider)!.listReferrals();
      _kFactorFuture = ref.read(apiServiceProvider)!.getViralCoefficient();
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
      body: FutureBuilder(
        future: Future.wait([_referralsFuture, _kFactorFuture]),
        builder: (context, AsyncSnapshot<List<dynamic>> snapshot) {
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

          final referrals = (snapshot.data?[0] as List?)?.cast<Map<String, dynamic>>() ?? [];
          final kFactorData = (snapshot.data?[1] as Map?)?.cast<String, dynamic>() ?? {};

          final kFactor = (kFactorData['kFactor'] as num?)?.toDouble() ?? 0.0;
          final uniqueInviters = kFactorData['uniqueInviters'] as int? ?? 0;

          Widget contentWidget;
          if (referrals.isEmpty) {
            contentWidget = const Center(
              child: Text(
                'No referrals tracked yet.',
                style: TextStyle(fontFamily: 'Inter', fontSize: 16),
              ),
            );
          } else {
            contentWidget = Wrap(
              spacing: 24,
              runSpacing: 24,
              children: referrals.map((r) {
                return _ReferralCard(referral: r);
              }).toList(),
            );
          }

          return SingleChildScrollView(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _KFactorCard(kFactor: kFactor, uniqueInviters: uniqueInviters),
                const SizedBox(height: 32),
                contentWidget,
              ],
            ),
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
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          child: Container(
            padding: const EdgeInsets.all(24),
            decoration: BoxDecoration(
              color: colors.surface.withValues(alpha: 0.6),
              border: Border.all(
                color: colors.onSurface.withValues(alpha: 0.1),
                width: 1.5,
              ),
              borderRadius: BorderRadius.circular(16),
            ),
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
  final int uniqueInviters;

  const _KFactorCard({required this.kFactor, required this.uniqueInviters});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 15.0, sigmaY: 15.0),
        child: Container(
          width: double.infinity,
          padding: const EdgeInsets.all(32),
          decoration: BoxDecoration(
            color: colors.surface.withValues(alpha: 0.03),
            border: Border.all(
              color: Colors.white.withValues(alpha: 0.1),
              width: 1.5,
            ),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Column(
            children: [
              Text(
                'Viral Coefficient (K-Factor)',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: colors.onSurface,
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
              const SizedBox(height: 8),
              Text(
                'across $uniqueInviters unique inviters',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: colors.onSurfaceVariant,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
