import 'package:flutter/material.dart';
import 'screens/onboarding.dart';
import 'screens/inbox.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'OHC Setup',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: InboxScreen(), // Changed to InboxScreen to show the new unified inbox by default for testing
    );
  }
}
