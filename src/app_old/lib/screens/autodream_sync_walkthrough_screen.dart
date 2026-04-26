import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class AutoDreamSyncWalkthroughScreen extends StatefulWidget {
  const AutoDreamSyncWalkthroughScreen({super.key});

  @override
  State<AutoDreamSyncWalkthroughScreen> createState() => _AutoDreamSyncWalkthroughScreenState();
}

class _AutoDreamSyncWalkthroughScreenState extends State<AutoDreamSyncWalkthroughScreen> {
  int _currentStep = 0;

  final List<Map<String, String>> _steps = [
    {
      'title': '1. Generate & Insert Vector',
      'description': 'Worker generates an intelligence vector and inserts it into Local SQLite DB with sync_status=\'pending\'.',
      'participant': 'Standalone AutoDreamWorker -> Local SQLite DB'
    },
    {
      'title': '2. Query Pending Vectors',
      'description': 'Sync Daemon periodically queries the Local SQLite DB for any vectors pending synchronization.',
      'participant': 'Sync Daemon -> Local SQLite DB'
    },
    {
      'title': '3. Return Batched Vectors',
      'description': 'Local SQLite DB returns the pending intelligence vectors in a batch format to the Sync Daemon.',
      'participant': 'Local SQLite DB -> Sync Daemon'
    },
    {
      'title': '4. Push over mTLS',
      'description': 'Sync Daemon securely pushes the batched vectors to the Cloud API Gateway using SPIFFE Identity over mTLS.',
      'participant': 'Sync Daemon -> Cloud API Gateway'
    },
    {
      'title': '5. Upsert to Global',
      'description': 'Cloud API Gateway upserts the received vectors into the Global Cloud PostgreSQL (autodream_memories).',
      'participant': 'Cloud API Gateway -> Cloud PostgreSQL'
    },
    {
      'title': '6. Acknowledge Success',
      'description': 'Cloud API Gateway sends an acknowledgment of successful storage back to the Sync Daemon.',
      'participant': 'Cloud API Gateway -> Sync Daemon'
    },
    {
      'title': '7. Update sync_status',
      'description': 'Sync Daemon updates the local records in SQLite, marking them as sync_status=\'synced\'.',
      'participant': 'Sync Daemon -> Local SQLite DB'
    },
  ];

  void _nextStep() {
    if (_currentStep < _steps.length - 1) {
      setState(() {
        _currentStep++;
      });
    }
  }

  void _previousStep() {
    if (_currentStep > 0) {
      setState(() {
        _currentStep--;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        title: const Text('AutoDream Sync Daemon Walkthrough', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Text(
                'Interactive Guide: Sync Lifecycle',
                style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
              ),
              const SizedBox(height: 24),
              ClipRRect(
                borderRadius: BorderRadius.circular(16),
                child: BackdropFilter(
                  filter: ImageFilter.compose(
                    outer: const ColorFilter.matrix(<double>[
                      1.168, -0.153, -0.015, 0, 0,
                      -0.046, 1.061, -0.015, 0, 0,
                      -0.046, -0.152, 1.198, 0, 0,
                      0, 0, 0, 1, 0,
                    ]),
                    inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                  ),
                  child: Container(
                    width: 600,
                    padding: const EdgeInsets.all(32.0),
                    decoration: BoxDecoration(
                      color: const Color.fromRGBO(255, 255, 255, 0.03),
                      borderRadius: BorderRadius.circular(16),
                      border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          _steps[_currentStep]['title']!,
                          style: const TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.blueAccent),
                        ),
                        const SizedBox(height: 16),
                        Text(
                          _steps[_currentStep]['description']!,
                          style: const TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.white),
                        ),
                        const SizedBox(height: 24),
                        Container(
                          padding: const EdgeInsets.all(12),
                          decoration: BoxDecoration(
                            color: Colors.black26,
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: Row(
                            children: [
                              const Icon(Icons.sync_alt, color: Colors.cyanAccent),
                              const SizedBox(width: 12),
                              Expanded(
                                child: Text(
                                  _steps[_currentStep]['participant']!,
                                  style: const TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.cyanAccent),
                                ),
                              ),
                            ],
                          ),
                        ),
                        const SizedBox(height: 32),
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Expanded(
                              child: Align(
                                alignment: Alignment.centerLeft,
                                child: ElevatedButton(
                                  onPressed: _currentStep > 0 ? _previousStep : null,
                                  child: const Text('Previous Step'),
                                ),
                              ),
                            ),
                            Text(
                              'Step ${_currentStep + 1} of ${_steps.length}',
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white70),
                            ),
                            Expanded(
                              child: Align(
                                alignment: Alignment.centerRight,
                                child: ElevatedButton(
                                  onPressed: _currentStep < _steps.length - 1 ? _nextStep : null,
                                  child: const Text('Next Step'),
                                ),
                              ),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
