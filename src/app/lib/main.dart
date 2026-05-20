import 'package:flutter/material.dart';
import 'screens/onboarding.dart';
import 'screens/add_offering.dart';

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
      home: OnboardingScreen(),
      routes: {
        '/onboarding': (context) => OnboardingScreen(),
        '/add-offering': (context) => AddOfferingScreen(),
      },
    );
  }
}
