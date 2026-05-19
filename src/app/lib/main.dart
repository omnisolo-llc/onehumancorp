import 'package:flutter/material.dart';
import 'screens/ai_prompt.dart';

void main() {
  runApp(const OHCApp());
}

class OHCApp extends StatelessWidget {
  const OHCApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'OHC Storefront Builder',
      theme: ThemeData(
        primarySwatch: Colors.blue,
        useMaterial3: true,
        fontFamily: 'Inter',
      ),
      home: const AIPromptScreen(),
    );
  }
}
