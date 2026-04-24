import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:url_launcher/url_launcher.dart';

import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/api_service.dart';

class PlanDetail {
  final String id;
  final String name;
  final double priceUsd;
  final String interval;
  final List<String> features;

  PlanDetail({
    required this.id,
    required this.name,
    required this.priceUsd,
    required this.interval,
    required this.features,
  });

  factory PlanDetail.fromJson(Map<String, dynamic> json) {
    return PlanDetail(
      id: json['id'] as String? ?? '',
      name: json['name'] as String? ?? '',
      priceUsd: (json['priceUsd'] ?? 0.0).toDouble(),
      interval: json['interval'] as String? ?? 'month',
      features: List<String>.from(json['features'] ?? []),
    );
  }
}

final pricingPlansProvider = FutureProvider.autoDispose<List<PlanDetail>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');

  final response = await api.getBillingPlans();

  if (response.statusCode != 200) {
    throw Exception('Failed to load plans: ${response.statusCode}');
  }

  final List<dynamic> data = jsonDecode(response.body);
  return data.map((json) => PlanDetail.fromJson(json)).toList();
});

class PricingScreen extends ConsumerWidget {
  const PricingScreen({super.key});

  Future<void> _upgradePlan(BuildContext context, WidgetRef ref, String planId) async {
    final api = ref.read(apiServiceProvider);
    if (api == null) return;

    try {
      final urlString = await api.createCheckoutSession(planId);
      final uri = Uri.parse(urlString);
      if (await canLaunchUrl(uri)) {
        await launchUrl(uri, mode: LaunchMode.externalApplication);
      } else {
        if (context.mounted) {
           ScaffoldMessenger.of(context).showSnackBar(
             const SnackBar(content: Text('Could not open checkout.')),
           );
        }
      }
    } catch (e) {
      if (context.mounted) {
         ScaffoldMessenger.of(context).showSnackBar(
           SnackBar(content: Text('Error: $e')),
         );
      }
    }
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final plansAsync = ref.watch(pricingPlansProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Choose Your Plan', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: plansAsync.when(
        loading: () => Center(child: CircularProgressIndicator(color: Theme.of(context).colorScheme.primary)),
        error: (err, stack) => Center(child: Text('Error: $err', style: const TextStyle(fontFamily: 'Inter'))),
        data: (plans) {
          return ListView.builder(
            padding: const EdgeInsets.all(24),
            itemCount: plans.length,
            itemBuilder: (context, index) {
              final plan = plans[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 24),
                child: GlassCard(
                  child: Padding(
                    padding: const EdgeInsets.all(24),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          plan.name,
                          style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                            fontFamily: 'Outfit',
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          '\$${plan.priceUsd.toStringAsFixed(0)} / ${plan.interval}',
                          style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                            fontFamily: 'Inter',
                            fontWeight: FontWeight.w600,
                            color: Theme.of(context).colorScheme.primary,
                          ),
                        ),
                        const SizedBox(height: 16),
                        ...plan.features.map((feature) => Padding(
                          padding: const EdgeInsets.only(bottom: 8),
                          child: Row(
                            children: [
                              Icon(Icons.check_circle, color: Theme.of(context).colorScheme.primary, size: 20),
                              const SizedBox(width: 8),
                              Expanded(
                                child: Text(feature, style: const TextStyle(fontFamily: 'Inter', fontSize: 16)),
                              ),
                            ],
                          ),
                        )),
                        const SizedBox(height: 24),
                        SizedBox(
                          width: double.infinity,
                          child: FilledButton(
                            style: FilledButton.styleFrom(
                              padding: const EdgeInsets.symmetric(vertical: 16),
                            ),
                            onPressed: () => _upgradePlan(context, ref, plan.id),
                            child: Text(plan.priceUsd == 0 ? 'Current Plan' : 'Upgrade to ${plan.name}', style: const TextStyle(fontFamily: 'Outfit', fontSize: 16, fontWeight: FontWeight.bold)),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              );
            },
          );
        },
      ),
    );
  }
}
