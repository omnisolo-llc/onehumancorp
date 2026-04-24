import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Release Notes', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'What\'s New',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 24),
            _buildReleaseCard(
              context,
              'Version 1.4.2',
              'April 24, 2026',
              [
                'Scout: Tool Integration Research [Q3] (#7786) - Identifies the observability gap where Hybrid KAIROS metrics lack Cloud vs Standalone mode differentiation.',
                'Added new Help Center and Interactive APIs to improve onboarding.',
                'Bug fixes and performance improvements.'
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildReleaseCard(BuildContext context, String version, String date, List<String> notes) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(child: Text(version, style: const TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold))),
                Text(date, style: TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.grey[400])),
              ],
            ),
            const Divider(height: 24),
            ...notes.map((note) => Padding(
              padding: const EdgeInsets.only(bottom: 8.0),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('• ', style: TextStyle(fontSize: 16)),
                  Expanded(child: Text(note, style: const TextStyle(fontFamily: 'Inter', fontSize: 14))),
                ],
              ),
            )),
          ],
        ),
      ),
    );
  }
}
