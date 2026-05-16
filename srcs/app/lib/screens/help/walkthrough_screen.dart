import 'package:flutter/material.dart';

class WalkthroughScreen extends StatelessWidget {
  const WalkthroughScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Walkthrough')),
      body: const Center(child: Text('Walkthrough')),
    );
  }
}
