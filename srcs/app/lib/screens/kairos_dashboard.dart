import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class KairosDashboardScreen extends ConsumerStatefulWidget {
  const KairosDashboardScreen({super.key});

  @override
  ConsumerState<KairosDashboardScreen> createState() => _KairosDashboardScreenState();
}

class _KairosDashboardScreenState extends ConsumerState<KairosDashboardScreen> {
  final List<String> _meshLogs = [];

  @override
  void initState() {
    super.initState();
    _connectToStream();
  }

  void _connectToStream() {
    Future.delayed(const Duration(seconds: 1), () {
      if (mounted) {
        setState(() {
          _meshLogs.insert(0, "Connected to KAIROS Orchestrator");
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF111111),
      appBar: AppBar(
        title: const Text('KAIROS Swarm Analytics', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Row(
          children: [
            Expanded(
              flex: 1,
              child: GlassCard(
                color: const Color.fromRGBO(255, 255, 255, 0.03),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('Shared Task Queue', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Inter')),
                    const SizedBox(height: 16),
                    Expanded(
                      child: ListView.builder(
                        itemCount: 3,
                        itemBuilder: (context, index) {
                          return ListTile(
                            title: Text('Task $index', style: const TextStyle(color: Colors.white)),
                            subtitle: const Text('Status: PENDING', style: TextStyle(color: Colors.grey)),
                          );
                        },
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              flex: 1,
              child: GlassCard(
                color: const Color.fromRGBO(255, 255, 255, 0.03),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('Teammate Mesh Stream', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Inter')),
                    const SizedBox(height: 16),
                    Expanded(
                      child: ListView.builder(
                        itemCount: _meshLogs.length,
                        itemBuilder: (context, index) {
                          return Text(_meshLogs[index], style: const TextStyle(color: Colors.green, fontFamily: 'monospace'));
                        },
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              flex: 1,
              child: GlassCard(
                color: const Color.fromRGBO(255, 255, 255, 0.03),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('AutoDream Memory', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Inter')),
                    const SizedBox(height: 16),
                    const Center(child: Text('Memory Consolidated', style: TextStyle(color: Colors.grey))),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
