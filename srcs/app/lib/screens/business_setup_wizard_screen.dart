import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
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
  final _ideaController = TextEditingController();

  late AnimationController _heroAnimationController;
  late Animation<double> _heroAnimation;

  late AnimationController _pulseAnimationController;
  late Animation<double> _pulseAnimation;

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

    _pulseAnimationController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 1),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 1.0, end: 1.05).animate(
      CurvedAnimation(parent: _pulseAnimationController, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _heroAnimationController.dispose();
    _pulseAnimationController.dispose();
    _ideaController.dispose();
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
              icon: const Icon(Icons.help_outline, color: Colors.white70, size: 28),
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
        return _buildGenerationScreen();
      case 2:
        return _buildReviewScreen(state);
      case 3:
        return _buildSuccessScreen(state);
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildWelcomeScreen() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text(
          'What are you building today?',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 30),
        GlassContainer(
          child: TextField(
            key: const Key('ideaField'),
            controller: _ideaController,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              hintText: 'e.g. A custom cake shop',
              hintStyle: TextStyle(color: Colors.white54),
              border: InputBorder.none,
              contentPadding: EdgeInsets.all(15),
            ),
            onChanged: (value) {
              ref.read(wizardProvider.notifier).setBusinessIdea(value);
            },
          ),
        ),
        const SizedBox(height: 30),
        ElevatedButton(
          onPressed: () {
            if (_ideaController.text.isNotEmpty) {
              _nextStep();
              // Simulate API delay, then go to review
              Future.delayed(const Duration(seconds: 3), () {
                if (mounted && ref.read(wizardProvider).currentStep == 1) {
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
          child: const Text('Next', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
        ),
      ],
    );
  }

  Widget _buildGenerationScreen() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        AnimatedBuilder(
          animation: _pulseAnimation,
          builder: (context, child) {
            return Transform.scale(
              scale: _pulseAnimation.value,
              child: const Icon(Icons.auto_awesome, color: Color(0xFF6B4EFF), size: 64),
            );
          },
        ),
        const SizedBox(height: 30),
        const Text(
          'Our AI (The Promoter) is designing your site...',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 24,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 20),
        const Center(
          child: CircularProgressIndicator(color: Color(0xFF6B4EFF)),
        )
      ],
    );
  }

  Widget _buildReviewScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text(
          'Review Your Site',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 20),
        Expanded(
          child: GlassContainer(
            child: Center(
              child: Text(
                'Preview for: ${state.businessIdea}',
                style: const TextStyle(color: Colors.white70, fontSize: 18),
                textAlign: TextAlign.center,
              ),
            ),
          ),
        ),
        const SizedBox(height: 30),
        ElevatedButton(
          onPressed: () async {
            await ref.read(wizardProvider.notifier).submitWizard();
          },
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF22C55E),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
          key: const Key('launchAIBtn'),
          child: const Text('Looks Good, Go Live', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
        ),
        const SizedBox(height: 15),
        TextButton(
          onPressed: _prevStep,
          child: const Text('Back', style: TextStyle(color: Colors.white54, fontSize: 16)),
        )
      ],
    );
  }

  Widget _buildSuccessScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text(
          '🎉 Success!',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 36,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 20),
        const Text(
          'Your business is live at:',
          style: TextStyle(color: Colors.white70, fontSize: 18),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 10),
        Text(
          state.generatedUrl ?? 'your-site.ohc.app',
          style: const TextStyle(color: Color(0xFF22C55E), fontSize: 20, fontWeight: FontWeight.bold),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 40),
        ElevatedButton.icon(
          onPressed: () {
            // Share logic
          },
          icon: const Icon(Icons.share, color: Colors.white),
          label: const Text('Share on Instagram', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFFE1306C),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: _nextStep,
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF6B4EFF),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
          child: const Text('Go to Dashboard', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
        ),
      ],
    );
  }
}
