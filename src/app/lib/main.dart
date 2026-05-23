import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'screens/onboarding.dart';
import 'screens/inbox.dart';
import 'screens/help_center.dart';
import 'screens/dashboard_screen.dart';
import 'screens/swarm_memory_screen.dart';
import 'screens/business_setup_wizard_screen.dart';

void main() {
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'OHC Setup',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      initialRoute: '/business_setup',
      routes: {
        '/': (context) => const MainNavigator(),
        '/business_setup': (context) => const BusinessSetupWizardScreen(),
        '/dashboard': (context) => DashboardScreen(),
        '/swarm_memory': (context) => SwarmMemoryScreen(),
      },
    );
  }
}

class MainNavigator extends StatelessWidget {
  const MainNavigator({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Dev Links'),
        actions: [
          IconButton(
            icon: Icon(Icons.dashboard),
            onPressed: () => Navigator.pushNamed(context, '/dashboard'),
            tooltip: 'Dashboard',
          ),
          IconButton(
            icon: Icon(Icons.memory),
            onPressed: () => Navigator.pushNamed(context, '/swarm_memory'),
            tooltip: 'Swarm Memory',
          ),
        ],
      ),
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
