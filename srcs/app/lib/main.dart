import 'dart:ui';
import 'package:flutter/material.dart';
import 'dart:async';

void main() {
  runApp(const OHCApp());
}

class OHCApp extends StatelessWidget {
  const OHCApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'One Human Corp',
      theme: ThemeData(
        useMaterial3: true,
        fontFamily: 'Inter',
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF6B4EFF),
          brightness: Brightness.dark,
        ),
      ),
      home: const OnboardingScreen(),
    );
  }
}

class GlassContainer extends StatelessWidget {
  final Widget child;
  const GlassContainer({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(20),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.1),
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: Colors.white.withOpacity(0.2)),
          ),
          padding: const EdgeInsets.all(20),
          child: child,
        ),
      ),
    );
  }
}

class OnboardingScreen extends StatefulWidget {
  const OnboardingScreen({super.key});

  @override
  State<OnboardingScreen> createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  int _step = 0;
  String _category = '';
  final _nameController = TextEditingController();

  void _nextStep() {
    setState(() {
      _step++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 375),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: _buildCurrentStep(),
          ),
        ),
      ),
    );
  }

  Widget _buildCurrentStep() {
    switch (_step) {
      case 0:
        return _buildCategorySelection();
      case 1:
        return _buildNameInput();
      case 2:
        return _buildLoadingScreen();
      default:
        return const DashboardScreen();
    }
  }

  Widget _buildCategorySelection() {
    final categories = ['Bake', 'Teach', 'Fix', 'Sell', 'Other'];
    return SingleChildScrollView(
      child: Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'What do you do?',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 40),
        Wrap(
          spacing: 10,
          runSpacing: 10,
          alignment: WrapAlignment.center,
          children: categories.map((cat) =>
            GestureDetector(
              onTap: () {
                setState(() => _category = cat);
                _nextStep();
              },
              child: GlassContainer(
                child: SizedBox(
                  width: 100,
                  height: 100,
                  child: Center(
                    child: Text(
                      cat,
                      style: const TextStyle(fontSize: 18, color: Colors.white),
                    ),
                  ),
                ),
              ),
            )
          ).toList(),
        ),
      ],
      ),
    );
  }

  Widget _buildNameInput() {
    return SingleChildScrollView(
      child: Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          "What's the name of your business?",
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 40),
        GlassContainer(
          child: TextField(
            controller: _nameController,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              hintText: 'Enter business name...',
              hintStyle: TextStyle(color: Colors.white54),
              border: InputBorder.none,
            ),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: () {
            if (_nameController.text.isNotEmpty) {
              _nextStep();
              // Simulate AI loading
              Future.delayed(const Duration(seconds: 3), () {
                if (mounted) {
                  _nextStep();
                }
              });
            }
          },
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF6B4EFF),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
          child: const Text('Continue', style: TextStyle(fontSize: 18)),
        ),
      ],
      ),
    );
  }

  Widget _buildLoadingScreen() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        const CircularProgressIndicator(color: Color(0xFF6B4EFF)),
        const SizedBox(height: 40),
        const Text(
          'Generating your store...',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 24,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 20),
        Text(
          'Setting up AI departments for $_category...',
          style: const TextStyle(color: Colors.white70),
        ),
      ],
    );
  }
}

class DashboardScreen extends StatelessWidget {
  const DashboardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SizedBox(height: 40),
        const Text(
          "You're live!",
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 20),
        GlassContainer(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: const [
              Text(
                'Let\'s add your first item.',
                style: TextStyle(fontSize: 18, color: Colors.white),
              ),
              SizedBox(height: 10),
              Text(
                'Your AI agents are ready to help.',
                style: TextStyle(color: Colors.white70),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
