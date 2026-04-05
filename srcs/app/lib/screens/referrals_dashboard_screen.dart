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

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _referralsFuture = ref.read(apiServiceProvider)!.listReferrals();
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
      body: FutureBuilder<List<Map<String, dynamic>>>(
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

          if (referrals.isEmpty) {
            return const Center(
              child: Text(
                'No referrals tracked yet.',
                style: TextStyle(fontFamily: 'Inter', fontSize: 16),
              ),
            );
          }

          int totalClicks = 0;
          int totalConversions = 0;
          for (var r in referrals) {
            totalClicks += (r['clicks'] as int?) ?? 0;
            totalConversions += (r['conversions'] as int?) ?? 0;
          }
          double conversionRate = totalClicks > 0 ? (totalConversions / totalClicks) * 100 : 0.0;

          return SingleChildScrollView(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _GrowthMetricsSummary(
                  totalClicks: totalClicks,
                  totalConversions: totalConversions,
                  conversionRate: conversionRate,
                ),
                const SizedBox(height: 32),
                Text(
                  'Individual Links',
                  style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                        fontFamily: 'Outfit',
                        fontWeight: FontWeight.bold,
                      ),
                ),
                const SizedBox(height: 24),
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

class _GrowthMetricsSummary extends StatelessWidget {
  final int totalClicks;
  final int totalConversions;
  final double conversionRate;

  const _GrowthMetricsSummary({
    required this.totalClicks,
    required this.totalConversions,
    required this.conversionRate,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(32),
          decoration: BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [
                colors.primaryContainer.withValues(alpha: 0.3),
                colors.secondaryContainer.withValues(alpha: 0.1),
              ],
            ),
            border: Border.all(
              color: colors.primary.withValues(alpha: 0.2),
              width: 1.5,
            ),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Wrap(
            alignment: WrapAlignment.spaceAround,
            spacing: 24,
            runSpacing: 24,
            children: [
              _StatColumn(label: 'Total Clicks', value: '$totalClicks', isHighlight: true),
              _StatColumn(label: 'Total Conversions', value: '$totalConversions', isHighlight: true),
              _StatColumn(
                label: 'Global Conversion Rate',
                value: '${conversionRate.toStringAsFixed(1)}%',
                isHighlight: true,
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _ReferralCard extends StatefulWidget {
  final Map<String, dynamic> referral;

  const _ReferralCard({required this.referral});

  @override
  State<_ReferralCard> createState() => _ReferralCardState();
}

class _ReferralCardState extends State<_ReferralCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final createdAt = DateTime.tryParse(widget.referral['createdAt'] ?? '') ?? DateTime.now();

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOut,
        child: ConstrainedBox(
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
                      'Ref: ${widget.referral['referralCode']}',
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
                  'User: ${widget.referral['userId']}',
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
                    _StatColumn(label: 'Clicks', value: '${widget.referral['clicks']}'),
                    _StatColumn(label: 'Conversions', value: '${widget.referral['conversions']}'),
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
        ),
      ),
    );
  }
}

class _StatColumn extends StatelessWidget {
  final String label;
  final String value;
  final bool isHighlight;

  const _StatColumn({
    required this.label,
    required this.value,
    this.isHighlight = false,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Column(
      children: [
        Text(
          value,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: isHighlight ? 32 : 24,
            fontWeight: FontWeight.bold,
            color: colors.primary,
          ),
        ),
        Text(
          label,
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: isHighlight ? 14 : 12,
            color: colors.onSurfaceVariant,
          ),
        ),
      ],
    );
  }
}
