import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  Widget _buildRelease(String version, String date, List<String> notes) {
    return Container(
      margin: const EdgeInsets.only(bottom: 24),
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(20.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    'Version $version',
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 22,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  Text(
                    date,
                    style: const TextStyle(
                      fontFamily: 'Inter',
                      fontSize: 14,
                      color: Colors.white60,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 16),
              ...notes.map((note) => Padding(
                    padding: const EdgeInsets.only(bottom: 8.0),
                    child: Row(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text('• ', style: TextStyle(color: Colors.white, fontSize: 16)),
                        Expanded(
                          child: Text(
                            note,
                            style: const TextStyle(
                              fontFamily: 'Inter',
                              fontSize: 15,
                              color: Colors.white70,
                              height: 1.4,
                            ),
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
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: ListView(
        padding: const EdgeInsets.all(24.0),
        children: [
          _buildRelease('1.4.0', 'April 2026', [
            'Added an interactive Help Portal to make finding answers easier.',
            'New AI Support Agent is now available to help manage customer requests automatically.',
            'Improved performance on mobile devices for the Dashboard.',
            'Fixed an issue where some receipts were not sending correctly.'
          ]),
          _buildRelease('1.3.2', 'March 2026', [
            'Added support for Apple Pay and Google Pay in the checkout flow.',
            'You can now view your website changes before publishing.',
            'Minor bug fixes and UI improvements.'
          ]),
        ],
      ),
    );
  }
}
