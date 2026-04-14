import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';
import 'dart:ui';
import 'package:intl/intl.dart';

class LandingPageExperimentsScreen extends ConsumerStatefulWidget {
  const LandingPageExperimentsScreen({super.key});

  @override
  ConsumerState<LandingPageExperimentsScreen> createState() =>
      _LandingPageExperimentsScreenState();
}

class _LandingPageExperimentsScreenState extends ConsumerState<LandingPageExperimentsScreen> {
  late Future<List<Map<String, dynamic>>> _experimentsFuture;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _experimentsFuture = ref.read(apiServiceProvider)!.listLandingPageExperiments();
    });
  }

  void _showCreateDialog() {
    final titleController = TextEditingController();
    final trafficSplitController = TextEditingController(text: '0.5');

    showDialog(
      context: context,
      barrierColor: Colors.black.withValues(alpha: 0.2),
      builder: (context) => Dialog(
        backgroundColor: Colors.transparent,
        elevation: 0,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(24),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(const <double>[
                1.787, -0.715, -0.072, 0, 0,
                -0.213, 1.285, -0.072, 0, 0,
                -0.213, -0.715, 1.928, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.6),
                borderRadius: BorderRadius.circular(24),
                border: Border.all(
                  color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.2),
                  width: 1.5,
                ),
              ),
              padding: const EdgeInsets.all(24),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    'New Growth Experiment',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 24,
                      fontWeight: FontWeight.w600,
                      color: Theme.of(context).colorScheme.onSurface,
                    ),
                  ),
                  const SizedBox(height: 24),
                  TextField(
                    controller: titleController,
                    decoration: const InputDecoration(
                      labelText: 'Experiment Title',
                      border: OutlineInputBorder(),
                    ),
                  ),
                  const SizedBox(height: 16),
                  TextField(
                    controller: trafficSplitController,
                    keyboardType: const TextInputType.numberWithOptions(decimal: true),
                    decoration: const InputDecoration(
                      labelText: 'Traffic Split (0.0 to 1.0)',
                      border: OutlineInputBorder(),
                    ),
                  ),
                  const SizedBox(height: 24),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      TextButton(
                        onPressed: () => Navigator.pop(context),
                        child: const Text('Cancel'),
                      ),
                      const SizedBox(width: 8),
                      FilledButton(
                        onPressed: () async {
                          final split = double.tryParse(trafficSplitController.text) ?? 0.5;
                          try {
                            await ref.read(apiServiceProvider)!.createLandingPageExperiment(
                                  titleController.text,
                                  split,
                                );
                            if (context.mounted) {
                              Navigator.pop(context);
                              _refresh();
                            }
                          } catch (e) {
                            if (context.mounted) {
                              ScaffoldMessenger.of(context).showSnackBar(
                                SnackBar(
                                  content: Text('Error: $e'),
                                  backgroundColor: Theme.of(context).colorScheme.error,
                                ),
                              );
                            }
                          }
                        },
                        child: const Text('Launch Experiment'),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    ).then((_) {
      titleController.dispose();
      trafficSplitController.dispose();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Growth Experiments'),
        actions: [
          IconButton(
            onPressed: _refresh,
            icon: const Icon(Icons.refresh),
            tooltip: 'Refresh experiments',
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: _showCreateDialog,
        icon: const Icon(Icons.add_chart),
        label: const Text('New Experiment'),
      ),
      body: FutureBuilder<List<Map<String, dynamic>>>(
        future: _experimentsFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(
              child: CircularProgressIndicator(
                color: Theme.of(context).colorScheme.primary,
              ),
            );
          }

          if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          }

          final experiments = snapshot.data ?? [];

          if (experiments.isEmpty) {
            return Center(
              child: Text(
                'No active growth experiments. Start A/B testing your funnel!',
                style: Theme.of(context).textTheme.titleMedium?.copyWith(
                      color: Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
              ),
            );
          }

          return ListView.separated(
            padding: const EdgeInsets.all(24),
            itemCount: experiments.length,
            separatorBuilder: (context, index) => const Divider(),
            itemBuilder: (context, index) {
              final exp = experiments[index];
              return ListTile(
                leading: CircleAvatar(
                  backgroundColor: Theme.of(context).colorScheme.primaryContainer,
                  child: Icon(Icons.science, color: Theme.of(context).colorScheme.primary),
                ),
                title: Text(
                  exp['title'] ?? 'Unknown',
                  style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                ),
                subtitle: Text(
                  'Traffic Split: ${(exp['trafficSplit'] * 100).toStringAsFixed(1)}% | Status: ${exp['status']}',
                  style: const TextStyle(fontFamily: 'Inter'),
                ),
                trailing: Chip(
                  label: const Text('Active'),
                  backgroundColor: Colors.green.withValues(alpha: 0.1),
                  labelStyle: const TextStyle(color: Colors.green),
                ),
              );
            },
          );
        },
      ),
    );
  }
}
