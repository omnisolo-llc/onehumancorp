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
            color: Colors.white.withAlpha(25),
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: Colors.white.withAlpha(51)),
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
  final _intentController = TextEditingController();

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
        return _buildIntake();
      case 1:
        return _buildLoadingScreen();
      case 2:
        return _buildReviewScreen();
      default:
        return const DashboardScreen();
    }
  }

  Widget _buildIntake() {
    return SingleChildScrollView(
      child: Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'What do you want to build today?',
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
            controller: _intentController,
            style: const TextStyle(color: Colors.white),
            maxLines: 3,
            decoration: const InputDecoration(
              hintText: 'e.g., I sell vegan cakes...',
              hintStyle: TextStyle(color: Colors.white54),
              border: InputBorder.none,
            ),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: () {
            if (_intentController.text.isNotEmpty) {
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
          child: const Text('Continue', style: TextStyle(fontSize: 18, color: Colors.white)),
        ),
      ],
      ),
    );
  }

  Widget _buildLoadingScreen() {
    return Stack(
      children: [
        Positioned.fill(
          child: Container(color: Colors.black.withAlpha(128)),
        ),
        Center(
          child: GlassContainer(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: const [
                CircularProgressIndicator(color: Color(0xFF6B4EFF)),
                SizedBox(height: 40),
                Text(
                  'Designing storefront...',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                SizedBox(height: 20),
                Text(
                  'Writing policies...',
                  style: TextStyle(color: Colors.white70),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildReviewScreen() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Review Your Business',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              children: [
                _buildPreviewSection('Storefront Preview', 'Vegan Cakes Online'),
                const SizedBox(height: 15),
                _buildPreviewSection('Policies', 'Standard Terms & Conditions'),
                const SizedBox(height: 15),
                _buildPreviewSection('Initial Products', '3 items generated'),
              ],
            ),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: () {
            _nextStep();
          },
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF22C55E),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
          child: const Text('Launch Business', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
        ),
      ],
    );
  }

  Widget _buildPreviewSection(String title, String subtitle) {
    return GlassContainer(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  title,
                  style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16, color: Colors.white),
                ),
                const SizedBox(height: 5),
                Text(
                  subtitle,
                  style: const TextStyle(color: Colors.white70),
                ),
              ],
            ),
          ),
          InkWell(
            onTap: () {},
            child: const SizedBox(
              width: 44,
              height: 44,
              child: Icon(Icons.edit, color: Colors.white54, size: 20),
            ),
          ),
        ],
      ),
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
        const SizedBox(height: 20),
        const Text(
          "Dashboard",
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
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
                'Revenue',
                style: TextStyle(fontSize: 14, color: Colors.white70),
              ),
              SizedBox(height: 5),
              Text(
                '\$0.00',
                style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold, color: Colors.white),
              ),
            ],
          ),
        ),
        const SizedBox(height: 20),
        GlassContainer(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: const [
              Text(
                'Pending Agent Approvals',
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
              ),
              SizedBox(height: 10),
              Text(
                'No pending approvals.',
                style: TextStyle(color: Colors.white70),
              ),
            ],
          ),
        ),
        const SizedBox(height: 20),
        GlassContainer(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: const [
              Text(
                'Recent Orders',
                style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
              ),
              SizedBox(height: 10),
              Text(
                'No orders yet.',
                style: TextStyle(color: Colors.white70),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
