import 'package:flutter/material.dart';

class ReleaseNote {
  final String version;
  final String title;
  final String description;
  final String date;

  ReleaseNote({
    required this.version,
    required this.title,
    required this.description,
    required this.date,
  });
}

class ReleaseNotesScreen extends StatelessWidget {
  final List<ReleaseNote> _notes = [
    ReleaseNote(
      version: 'v0.4.44',
      title: 'Help is here! 🚀',
      description: 'We just added a new Help Center, Video Tutorials, and an AI Help Assistant to answer all your questions instantly. We also added tooltips to explain everything on the screen.',
      date: 'Today',
    ),
    ReleaseNote(
      version: 'v0.4.43',
      title: 'Easier Payments',
      description: 'Accepting Apple Pay is now faster and more reliable. Customers can check out in just one tap.',
      date: 'Last Week',
    ),
    ReleaseNote(
      version: 'v0.4.42',
      title: 'AI Helper Updates',
      description: 'Your Customer Success AI now understands return policies better and will give customers more accurate answers.',
      date: '2 Weeks Ago',
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFFF8FAFC),
      appBar: AppBar(
        title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit', color: Colors.black87, fontWeight: FontWeight.bold)),
        backgroundColor: Colors.white,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.black87),
      ),
      body: Center(
        child: Container(
          maxWidth: 600,
          child: ListView.builder(
            padding: const EdgeInsets.all(24),
            itemCount: _notes.length,
            itemBuilder: (context, index) {
              return _buildReleaseNoteCard(_notes[index]);
            },
          ),
        ),
      ),
    );
  }

  Widget _buildReleaseNoteCard(ReleaseNote note) {
    return Container(
      margin: const EdgeInsets.only(bottom: 24),
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color(0xFFE2E8F0)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.02),
            blurRadius: 10,
            offset: const Offset(0, 4),
          )
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                decoration: BoxDecoration(
                  color: const Color(0xFFEFF6FF),
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Text(
                  note.version,
                  style: const TextStyle(color: Color(0xFF2563EB), fontWeight: FontWeight.bold, fontSize: 12),
                ),
              ),
              Text(
                note.date,
                style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 12, fontWeight: FontWeight.bold),
              )
            ],
          ),
          const SizedBox(height: 16),
          Text(
            note.title,
            style: const TextStyle(
              fontFamily: 'Outfit',
              fontSize: 20,
              fontWeight: FontWeight.bold,
              color: Color(0xFF0F172A),
            ),
          ),
          const SizedBox(height: 8),
          Text(
            note.description,
            style: const TextStyle(
              fontFamily: 'Inter',
              fontSize: 16,
              color: Color(0xFF475569),
              height: 1.5,
            ),
          ),
          const SizedBox(height: 16),
          InkWell(
            onTap: () {
              // Open full changelog on website
            },
            child: const Text(
              'Read full changelog online →',
              style: TextStyle(color: Color(0xFF0EA5E9), fontWeight: FontWeight.bold),
            ),
          )
        ],
      ),
    );
  }
}

extension on Container {
  get maxWidth => 600.0;
}
