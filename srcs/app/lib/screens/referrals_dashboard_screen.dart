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
  late Future<Map<String, dynamic>> _viralCoefficientFuture;

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
      body: FutureBuilder<List<dynamic>>(
        future: Future.wait([_referralsFuture, _viralCoefficientFuture]),
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

          final data = snapshot.data;
          if (data == null) {
            return const SizedBox();
          }

          final referrals = data[0] as List<Map<String, dynamic>>;
          final viralData = data[1] as Map<String, dynamic>;

          return SingleChildScrollView(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _ViralCoefficientHeader(data: viralData),
                const SizedBox(height: 32),
                const Text(
                  'Recent Referrals',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 16),
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

class _ViralCoefficientHeader extends StatelessWidget {
  final Map<String, dynamic> data;

  const _ViralCoefficientHeader({required this.data});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final kFactor = (data['kFactor'] ?? 0.0) as num;
    final isViral = kFactor >= 1.0;

    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(32),
          decoration: BoxDecoration(
            color: colors.surface.withValues(alpha: 0.6),
            border: Border.all(
              color: isViral
                  ? Colors.green.withValues(alpha: 0.3)
                  : colors.onSurface.withValues(alpha: 0.1),
              width: 1.5,
            ),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Wrap(
            spacing: 24,
            runSpacing: 24,
            alignment: WrapAlignment.spaceAround,
            children: [
              _StatColumn(
                label: 'Viral Coefficient (K-Factor)',
                value: kFactor.toStringAsFixed(2),
                valueColor: isViral ? Colors.green : colors.primary,
              ),
              _StatColumn(
                label: 'Total Conversions',
                value: '${data['totalConversions'] ?? 0}',
              ),
              _StatColumn(
                label: 'Unique Inviters',
                value: '${data['uniqueInviters'] ?? 0}',
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _StatColumn extends StatelessWidget {
  final String label;
  final String value;
  final Color? valueColor;

  const _StatColumn({required this.label, required this.value, this.valueColor});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          value,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: valueColor ?? colors.primary,
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
