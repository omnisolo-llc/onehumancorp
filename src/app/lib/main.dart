import 'package:flutter/material.dart';
import 'screens/onboarding.dart';
import 'screens/inbox.dart';
import 'screens/help_center.dart';

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
      home: const MainNavigator(),
    );
  }
}

class MainNavigator extends StatelessWidget {
  const MainNavigator({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: OnboardingScreen(),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Navigator.push(context, MaterialPageRoute(builder: (context) => HelpCenterScreen()));
        },
        backgroundColor: const Color(0xFF0EA5E9),
        child: const Icon(Icons.help_outline, color: Colors.white),
        tooltip: 'Help Center',
      ),
    );
  }
}
