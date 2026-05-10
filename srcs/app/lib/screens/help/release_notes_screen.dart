import 'package:flutter/material.dart';
import '../../main.dart'; // For GlassContainer

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Release Notes', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                _buildReleaseCard('v0.4.41', 'May 2026', [
                  'Enhance multi-tenant onboarding flow tests for the Welcome Checklist to ensure reliable scaling.',
                  'Bolster Standalone Wizard state test coverage for improved offline reliability and progressive disclosure validation.',
                ]),
                _buildReleaseCard('v0.4.37', 'May 2026', [
                  'Interop Mesh Comprehensive Test Coverage to improve distributed lock resilience.',
                  'Ensured graceful interop mesh protocol handling for malformed offline mesh payloads.',
                ]),
                const SizedBox(height: 20),
                _buildReleaseCard('v0.4.33', 'May 2024', [
                  'Added in-app help center and searchable knowledge base.',
                  'Introduced contextual tooltips to clarify confusing options.',
                  'New floating AI Help Chat to assist you anywhere in the app.',
                ]),
                const SizedBox(height: 20),
                _buildReleaseCard('v0.4.32', 'April 2024', [
                  'Redesigned the Business Setup Wizard for a smoother onboarding.',
                  'Performance improvements and bug fixes.',
                ]),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildReleaseCard(String version, String date, List<String> notes) {
    return GlassContainer(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                version,
                style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white),
              ),
              Text(
                date,
                style: const TextStyle(color: Colors.white70),
              ),
            ],
          ),
          const SizedBox(height: 15),
          ...notes.map((note) => Padding(
                padding: const EdgeInsets.only(bottom: 8.0),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('• ', style: TextStyle(color: Colors.white, fontSize: 16)),
                    Expanded(child: Text(note, style: const TextStyle(color: Colors.white, fontSize: 16))),
                  ],
                ),
              )),
        ],
      ),
    );
  }
}
