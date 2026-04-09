import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:ui';
import 'package:ohc_app/services/api_service.dart';

class ViralCoefficientWidget extends ConsumerStatefulWidget {
  const ViralCoefficientWidget({super.key});

  @override
  ConsumerState<ViralCoefficientWidget> createState() =>
      _ViralCoefficientWidgetState();
}

class _ViralCoefficientWidgetState
    extends ConsumerState<ViralCoefficientWidget> {
  late Future<Map<String, dynamic>> _viralFuture;

  @override
  void initState() {
    super.initState();
    _viralFuture = ref.read(apiServiceProvider)!.getViralCoefficient();
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(24),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            border: Border.all(color: colorScheme.outline.withValues(alpha: 0.2)),
            borderRadius: BorderRadius.circular(16),
          ),
          child: FutureBuilder<Map<String, dynamic>>(
            future: _viralFuture,
            builder: (context, snapshot) {
              if (snapshot.connectionState == ConnectionState.waiting) {
                return const Center(child: CircularProgressIndicator());
              }

              if (snapshot.hasError) {
                return Center(
                  child: Text(
                    'Failed to load K-Factor',
                    style: TextStyle(color: colorScheme.error),
                  ),
                );
              }

              final data = snapshot.data!;
              final kFactor = data['kFactor'] as num;
              final totalReferrals = data['totalReferrals'] as num;
              final totalConversions = data['totalConversions'] as num;
              final uniqueInviters = data['uniqueInviters'] as num;

              return Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(Icons.trending_up, color: colorScheme.primary),
                      const SizedBox(width: 8),
                      Text(
                        'Viral Coefficient (K-Factor)',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: colorScheme.onSurface,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      _StatItem(
                        label: 'K-Factor',
                        value: kFactor.toStringAsFixed(2),
                        valueColor: colorScheme.primary,
                      ),
                      _StatItem(
                        label: 'Total Referrals',
                        value: totalReferrals.toString(),
                      ),
                      _StatItem(
                        label: 'Conversions',
                        value: totalConversions.toString(),
                      ),
                      _StatItem(
                        label: 'Unique Inviters',
                        value: uniqueInviters.toString(),
                      ),
                    ],
                  ),
                ],
              );
            },
          ),
        ),
      ),
    );
  }
}

class _StatItem extends StatelessWidget {
  final String label;
  final String value;
  final Color? valueColor;

  const _StatItem({required this.label, required this.value, this.valueColor});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: 12,
            color: colorScheme.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 4),
        Text(
          value,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 24,
            fontWeight: FontWeight.bold,
            color: valueColor ?? colorScheme.onSurface,
          ),
        ),
      ],
    );
  }
}
