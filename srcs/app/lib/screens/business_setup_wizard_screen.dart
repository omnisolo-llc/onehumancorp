import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/wizard_provider.dart';
import '../main.dart'; // For GlassContainer and DashboardScreen

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> with TickerProviderStateMixin {
  final _companyNameController = TextEditingController();

  late AnimationController _pulseAnimationController;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
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
    _pulseAnimationController.dispose();
    _companyNameController.dispose();
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

    if (state.currentStep == 3) {
      return const DashboardScreen();
    }

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 375), // Enforcing 375px mobile-first
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: _buildCurrentStep(state.currentStep, state),
          ),
        ),
      ),
    );
  }

  Widget _buildCurrentStep(int step, WizardState state) {
    switch (step) {
      case 0:
        return _buildCategoryScreen(state);
      case 1:
        return _buildNameScreen(state);
      case 2:
        return _buildLoadingScreen(state);
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildCategoryScreen(WizardState state) {
    final categories = [
      {'icon': Icons.cake, 'label': 'Bake'},
      {'icon': Icons.school, 'label': 'Teach'},
      {'icon': Icons.build, 'label': 'Fix'},
      {'icon': Icons.storefront, 'label': 'Sell'},
      {'icon': Icons.more_horiz, 'label': 'Other'},
    ];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text(
          'What do you do?',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 30),
        Expanded(
          child: GridView.builder(
            gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: 2,
              crossAxisSpacing: 15,
              mainAxisSpacing: 15,
              childAspectRatio: 1.0,
            ),
            itemCount: categories.length,
            itemBuilder: (context, index) {
              final cat = categories[index];
              final isSelected = state.category == cat['label'];
              return GestureDetector(
                onTap: () {
                  ref.read(wizardProvider.notifier).setCategory(cat['label'] as String);
                  _nextStep(); // Auto-advance for frictionlessness
                },
                child: GlassContainer(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        cat['icon'] as IconData,
                        size: 40,
                        color: isSelected ? const Color(0xFF6B4EFF) : Colors.white70,
                      ),
                      const SizedBox(height: 10),
                      Text(
                        cat['label'] as String,
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 16,
                          fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                          color: isSelected ? Colors.white : Colors.white70,
                        ),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      ],
    );
  }

  Widget _buildNameScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text(
          'What\'s the name of your business?',
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
            key: const Key('companyNameField'),
            controller: _companyNameController,
            style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontSize: 18),
            decoration: const InputDecoration(
              hintText: 'e.g. Maya\'s Bakes',
              hintStyle: TextStyle(color: Colors.white54),
              border: InputBorder.none,
            ),
            onChanged: (value) {
              ref.read(wizardProvider.notifier).setCompanyName(value);
            },
            onSubmitted: (_) async {
              final currentState = ref.read(wizardProvider);
              if (currentState.companyName != null && currentState.companyName!.isNotEmpty) {
                 _nextStep();
                 await ref.read(wizardProvider.notifier).submitWizard();
              }
            },
          ),
        ),
        const SizedBox(height: 30),
        Row(
          children: [
            Expanded(
              flex: 1,
              child: ElevatedButton(
                onPressed: _prevStep,
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.transparent,
                  side: const BorderSide(color: Colors.white54),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Back', style: TextStyle(fontFamily: 'Inter', fontSize: 18, color: Colors.white)),
              ),
            ),
            const SizedBox(width: 15),
            Expanded(
              flex: 2,
              child: ElevatedButton(
                onPressed: () async {
                  final currentState = ref.read(wizardProvider);
                  if (currentState.companyName != null && currentState.companyName!.isNotEmpty) {
                    _nextStep();
                    await ref.read(wizardProvider.notifier).submitWizard();
                  }
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF6B4EFF),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Next', style: TextStyle(fontFamily: 'Inter', fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildLoadingScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        AnimatedBuilder(
          animation: _pulseAnimation,
          builder: (context, child) {
            return Transform.scale(
              scale: _pulseAnimation.value,
              child: const Icon(Icons.auto_awesome, size: 80, color: Color(0xFF6B4EFF)),
            );
          },
        ),
        const SizedBox(height: 30),
        const Text(
          'Generating your store...',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 20),
        const Text(
          'AI Agents are designing your site, configuring your inbox, and setting up your payments.',
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: 16,
            color: Colors.white70,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 40),
        if (state.errorMessage != null)
           Container(
             padding: const EdgeInsets.all(15),
             decoration: BoxDecoration(
               color: Colors.red.withOpacity(0.2),
               borderRadius: BorderRadius.circular(10),
               border: Border.all(color: Colors.red),
             ),
             child: Column(
               children: [
                 Text(
                   state.errorMessage!,
                   style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
                   textAlign: TextAlign.center,
                 ),
                 const SizedBox(height: 10),
                 ElevatedButton(
                   onPressed: () async {
                     await ref.read(wizardProvider.notifier).submitWizard();
                   },
                   style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.red,
                   ),
                   child: const Text('Retry', style: TextStyle(color: Colors.white)),
                 )
               ]
             )
           )
      ],
    );
  }
}
