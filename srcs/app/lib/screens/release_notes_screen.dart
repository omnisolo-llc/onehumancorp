import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNote {
  final String version;
  final String date;
  final List<String> changes;

  const ReleaseNote({
    required this.version,
    required this.date,
    required this.changes,
  });
}

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  static const List<ReleaseNote> notes = [
    ReleaseNote(
      version: 'v1.4.0',
      date: 'October 24, 2023',
      changes: [
        'Added a new Help Center with interactive search.',
        'Global "Ask anything" AI chat button available on all screens.',
        'Improved storefront load times by 40%.',
      ],
    ),
    ReleaseNote(
      version: 'v1.3.2',
      date: 'October 10, 2023',
      changes: [
        'Fixed an issue where Stripe Webhooks failed on retries.',
        'Updated UI to use premium glassmorphism tokens.',
      ],
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Release Notes', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            if (context.canPop()) {
              context.pop();
            } else {
              context.go('/help');
            }
          },
        ),
      ),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF1E1E28), Color(0xFF0F0F14)],
          ),
        ),
        child: ListView.builder(
          padding: const EdgeInsets.all(16.0),
          itemCount: notes.length,
          itemBuilder: (context, index) {
            final note = notes[index];
            return Padding(
              padding: const EdgeInsets.only(bottom: 16.0),
              child: GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text(
                            note.version,
                            style: const TextStyle(
                              color: Colors.white,
                              fontSize: 20,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          Text(
                            note.date,
                            style: const TextStyle(color: Colors.white54),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      ...note.changes.map((change) => Padding(
                            padding: const EdgeInsets.only(bottom: 8.0),
                            child: Row(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                const Text('• ', style: TextStyle(color: Colors.indigoAccent, fontSize: 18)),
                                Expanded(
                                  child: Text(
                                    change,
                                    style: const TextStyle(color: Colors.white70, fontSize: 14),
                                  ),
                                ),
                              ],
                            ),
                          )),
                    ],
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
