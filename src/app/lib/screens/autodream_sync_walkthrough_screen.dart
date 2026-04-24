import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/interactive_walkthrough.dart';

class AutoDreamSyncWalkthroughScreen extends StatelessWidget {
  const AutoDreamSyncWalkthroughScreen({super.key});

  final List<WalkthroughStep> _steps = const [
    WalkthroughStep(
      title: '1. Generate & Insert Vector',
      description: 'Worker generates an intelligence vector and inserts it into Local SQLite DB with sync_status=\'pending\'.',
      participant: 'Standalone AutoDreamWorker -> Local SQLite DB'
    ),
    WalkthroughStep(
      title: '2. Query Pending Vectors',
      description: 'Sync Daemon periodically queries the Local SQLite DB for any vectors pending synchronization.',
      participant: 'Sync Daemon -> Local SQLite DB'
    ),
    WalkthroughStep(
      title: '3. Return Batched Vectors',
      description: 'Local SQLite DB returns the pending intelligence vectors in a batch format to the Sync Daemon.',
      participant: 'Local SQLite DB -> Sync Daemon'
    ),
    WalkthroughStep(
      title: '4. Push over mTLS',
      description: 'Sync Daemon securely pushes the batched vectors to the Cloud API Gateway using SPIFFE Identity over mTLS.',
      participant: 'Sync Daemon -> Cloud API Gateway'
    ),
    WalkthroughStep(
      title: '5. Upsert to Global',
      description: 'Cloud API Gateway upserts the received vectors into the Global Cloud PostgreSQL (autodream_memories).',
      participant: 'Cloud API Gateway -> Cloud PostgreSQL'
    ),
    WalkthroughStep(
      title: '6. Acknowledge Success',
      description: 'Cloud API Gateway sends an acknowledgment of successful storage back to the Sync Daemon.',
      participant: 'Cloud API Gateway -> Sync Daemon'
    ),
    WalkthroughStep(
      title: '7. Update sync_status',
      description: 'Sync Daemon updates the local records in SQLite, marking them as sync_status=\'synced\'.',
      participant: 'Sync Daemon -> Local SQLite DB'
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        title: const Text('AutoDream Sync Daemon Walkthrough', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: InteractiveWalkthrough(
        title: 'AutoDream Sync Daemon Walkthrough',
        subtitle: 'Interactive Guide: Sync Lifecycle',
        steps: _steps,
      ),
    );
  }
}
