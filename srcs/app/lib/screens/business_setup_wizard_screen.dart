import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:confetti/confetti.dart';

import '../providers/wizard_provider.dart';
import '../main.dart'; // For GlassContainer
import '../widgets/contextual_tooltip.dart';
import '../widgets/walkthrough_overlay.dart';
import 'help/help_center_screen.dart';

enum EnvironmentMode { cloud, standaloneDesktop }

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  final EnvironmentMode environmentMode;

  const BusinessSetupWizardScreen({
    super.key,
    this.environmentMode = EnvironmentMode.cloud,
  });

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> with TickerProviderStateMixin {
  final _companyNameController = TextEditingController();
  final _categoryController = TextEditingController();

  late AnimationController _heroAnimationController;
  late Animation<double> _heroAnimation;
  late ConfettiController _confettiController;
  bool _showConfetti = false;

  @override
  void initState() {
    super.initState();
    _heroAnimationController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    )..repeat(reverse: true);
    _heroAnimation = Tween<double>(begin: -10, end: 10).animate(
      CurvedAnimation(parent: _heroAnimationController, curve: Curves.easeInOut),
    );
    _confettiController = ConfettiController(duration: const Duration(seconds: 3));

    WidgetsBinding.instance.addPostFrameCallback((_) {
      final state = ref.read(wizardProvider);
      if (state.companyName != null) {
        _companyNameController.text = state.companyName!;
      }
      if (state.industry != null) {
        _categoryController.text = state.industry!;
      }
    });
  }

  @override
  void dispose() {
    _heroAnimationController.dispose();
    _companyNameController.dispose();
    _categoryController.dispose();
    _confettiController.dispose();
    super.dispose();
  }

  void _nextStep() {
    ref.read(wizardProvider.notifier).nextStep();
  }

  void _prevStep() {
    ref.read(wizardProvider.notifier).prevStep();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(wizardProvider);

    if (state.currentStep == 4) {
      return const DashboardScreen();
    }

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Stack(
        children: [
          Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 400),
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: _buildCurrentStep(state.currentStep, state),
              ),
            ),
          ),
          Positioned(
            top: 40,
            right: 20,
            child: IconButton(
              icon: const Icon(Icons.help_outline, color: Colors.white, size: 30),
              onPressed: () {
                Navigator.push(context, MaterialPageRoute(builder: (context) => const HelpCenterScreen()));
              },
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCurrentStep(int step, WizardState state) {
    switch (step) {
      case 0:
        return _buildWelcomeScreen();
      case 1:
        return _buildBasicInfoScreen(state);
      case 2:
        return _buildImageUploadScreen(state);
      case 3:
        return _buildLoadingScreen(state);
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildWelcomeScreen() {
    return SingleChildScrollView(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          AnimatedBuilder(
            animation: _heroAnimation,
            builder: (context, child) {
              return Transform.translate(
                offset: Offset(0, _heroAnimation.value),
                child: child,
              );
            },
            child: const Icon(Icons.rocket_launch, size: 80, color: Color(0xFF6B4EFF)),
          ),
          const SizedBox(height: 30),
          const Text(
            'Welcome to One Human Corp',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 32,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 20),
          const Text(
            'Create your account to start your business.',
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 16,
              color: Colors.white70,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 40),
          ElevatedButton(
            onPressed: _nextStep,
            style: ElevatedButton.styleFrom(
              minimumSize: const Size(double.infinity, 44),
              backgroundColor: const Color(0xFF6B4EFF),
              padding: const EdgeInsets.symmetric(vertical: 20),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(15),
              ),
            ),
            child: const Text('Start My Business →', style: TextStyle(fontSize: 18, color: Colors.white)),
          ),
        ],
      ),
    );
  }

  Widget _buildBasicInfoScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Tell us about your business',
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
            child: GlassContainer(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Business Name', style: TextStyle(color: Colors.white70)),
                  const SizedBox(height: 10),
                  TextField(
                    key: const Key('companyNameField'),
                    controller: _companyNameController,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(
                      hintText: 'e.g. Maya\'s Cakes',
                      hintStyle: TextStyle(color: Colors.white38),
                      border: OutlineInputBorder(),
                    ),
                    onChanged: (val) => ref.read(wizardProvider.notifier).updateBusinessProfile(companyName: val),
                  ),
                  const SizedBox(height: 20),
                  const Text('Category', style: TextStyle(color: Colors.white70)),
                  const SizedBox(height: 10),
                  TextField(
                    key: const Key('categoryField'),
                    controller: _categoryController,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(
                      hintText: 'e.g. Bakery, Handyman, Tutor',
                      hintStyle: TextStyle(color: Colors.white38),
                      border: OutlineInputBorder(),
                    ),
                    onChanged: (val) => ref.read(wizardProvider.notifier).updateBusinessProfile(industry: val),
                  ),
                ],
              ),
            ),
          ),
        ),
        const SizedBox(height: 20),
        Row(
          children: [
            Expanded(
              flex: 1,
              child: ElevatedButton(
                onPressed: _prevStep,
                style: ElevatedButton.styleFrom(
                  minimumSize: const Size(double.infinity, 44),
                  backgroundColor: Colors.transparent,
                  side: const BorderSide(color: Colors.white54),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Back', style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
            const SizedBox(width: 15),
            Expanded(
              flex: 2,
              child: ElevatedButton(
                onPressed: _nextStep,
                style: ElevatedButton.styleFrom(
                  minimumSize: const Size(double.infinity, 44),
                  backgroundColor: const Color(0xFF6B4EFF),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Next →', style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildImageUploadScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Upload a Photo',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'Show us what you sell! Our AI will use this to design your store.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: Center(
            child: InkWell(
              key: const Key('imageUploadBtn'),
              onTap: () {
                // Mock image upload
                ref.read(wizardProvider.notifier).setUploadedImagePath('mock/path/image.jpg');
              },
              child: GlassContainer(
                child: Container(
                  height: 200,
                  width: double.infinity,
                  decoration: BoxDecoration(
                    border: Border.all(color: state.uploadedImagePath != null ? Colors.green : Colors.white38, width: 2),
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        state.uploadedImagePath != null ? Icons.check_circle : Icons.camera_alt,
                        size: 60,
                        color: state.uploadedImagePath != null ? Colors.green : Colors.white54,
                      ),
                      const SizedBox(height: 10),
                      Text(
                        state.uploadedImagePath != null ? 'Image Uploaded!' : 'Tap to Upload',
                        style: TextStyle(color: state.uploadedImagePath != null ? Colors.green : Colors.white70, fontSize: 18),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
        const SizedBox(height: 20),
        Row(
          children: [
            Expanded(
              flex: 1,
              child: ElevatedButton(
                onPressed: _prevStep,
                style: ElevatedButton.styleFrom(
                  minimumSize: const Size(double.infinity, 44),
                  backgroundColor: Colors.transparent,
                  side: const BorderSide(color: Colors.white54),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Back', style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
            const SizedBox(width: 15),
            Expanded(
              flex: 2,
              child: ElevatedButton(
                key: const Key('launchAIBtn'),
                onPressed: () {
                  _nextStep();
                  // Simulate AI generation process and finish
                  Future.delayed(const Duration(seconds: 3), () {
                    ref.read(wizardProvider.notifier).submitWizard();
                    if (mounted) {
                      setState(() {
                        _showConfetti = true;
                        _confettiController.play();
                      });
                      Future.delayed(const Duration(seconds: 2), () {
                        if (mounted) {
                           _nextStep();
                        }
                      });
                    }
                  });
                },
                style: ElevatedButton.styleFrom(
                  minimumSize: const Size(double.infinity, 44),
                  backgroundColor: const Color(0xFF22C55E),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Generate My Store ✨', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildLoadingScreen(WizardState state) {
    return Stack(
      children: [
        Center(
          child: ClipRRect(
            borderRadius: BorderRadius.circular(20),
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
              child: Container(
                width: 300,
                padding: const EdgeInsets.all(40),
                decoration: BoxDecoration(
                  color: Colors.white.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(20),
                  border: Border.all(color: Colors.white.withOpacity(0.2)),
                ),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    if (_showConfetti)
                      const Icon(Icons.check_circle, size: 80, color: Colors.green)
                    else
                      const CircularProgressIndicator(color: Color(0xFF6B4EFF), strokeWidth: 4),
                    const SizedBox(height: 30),
                    Text(
                      _showConfetti ? 'Store Created!' : 'Our AI is building your business...',
                      style: const TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 22,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                      textAlign: TextAlign.center,
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
        Align(
          alignment: Alignment.topCenter,
          child: ConfettiWidget(
            confettiController: _confettiController,
            blastDirectionality: BlastDirectionality.explosive,
            shouldLoop: false,
            colors: const [Colors.green, Colors.blue, Colors.pink, Colors.orange, Colors.purple],
          ),
        ),
      ],
    );
  }
}
