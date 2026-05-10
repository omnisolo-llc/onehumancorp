import 'package:flutter/material.dart';

class InteractiveWalkthroughScreen extends StatefulWidget {
  const InteractiveWalkthroughScreen({super.key});

  @override
  State<InteractiveWalkthroughScreen> createState() => _InteractiveWalkthroughScreenState();
}

class _InteractiveWalkthroughScreenState extends State<InteractiveWalkthroughScreen> {
  int _currentStep = 0;

  final List<Map<String, String>> _steps = [
    {
      'title': 'Set up your store',
      'description': 'Click here to add photos and prices for what you sell.',
    },
    {
      'title': 'Accept your first payment',
      'description': 'Connect your bank so you can get paid by customers.',
    },
    {
      'title': 'Activate your AI Support Agent',
      'description': 'Turn on your AI helper to automate customer support.',
    },
    {
      'title': 'You\'re all set!',
      'description': 'Your store is ready. You can always find more help in the Help Center.',
    },
  ];

  void _nextStep() {
    if (_currentStep < _steps.length - 1) {
      setState(() {
        _currentStep++;
      });
    } else {
      Navigator.of(context).pop();
    }
  }

  void _skipTour() {
    Navigator.of(context).pop();
  }

  @override
  Widget build(BuildContext context) {
    // Scaffold background is completely transparent now to act as overlay over the main app
    return Scaffold(
      backgroundColor: Colors.transparent,
      body: Stack(
        children: [
          _buildOverlayCard(context),
        ],
      ),
    );
  }

  Widget _buildOverlayCard(BuildContext context) {
    // Overlay highlight + speech bubble system
    Alignment alignment = Alignment.topLeft;
    EdgeInsets padding = const EdgeInsets.only(top: 100, left: 20);

    if (_currentStep == 1) {
      alignment = Alignment.topCenter;
      padding = const EdgeInsets.only(top: 100);
    } else if (_currentStep >= 2) {
      alignment = Alignment.bottomRight;
      padding = const EdgeInsets.only(bottom: 100, right: 20);
    }

    return Align(
      alignment: alignment,
      child: Padding(
        padding: padding,
        child: Container(
          width: 335,
          decoration: BoxDecoration(
            color: const Color(0xFF1E293B),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color(0xFF0EA5E9), width: 2), // Highlight border
            boxShadow: [
              BoxShadow(
                color: Colors.black.withAlpha(50),
                blurRadius: 10,
                offset: const Offset(0, 5),
              ),
            ],
          ),
          padding: const EdgeInsets.all(20),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                _steps[_currentStep]['title']!,
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 16),
              Text(
                _steps[_currentStep]['description']!,
                style: const TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: Color(0xFF94A3B8),
                ),
              ),
              const SizedBox(height: 16),
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  TextButton(
                    onPressed: _skipTour,
                    child: const Text('Skip tour', style: TextStyle(color: Colors.white)),
                  ),
                  const SizedBox(width: 12),
                  ElevatedButton(
                    onPressed: _nextStep,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: const Color(0xFF0EA5E9),
                      foregroundColor: Colors.white,
                    ),
                    child: Text(_currentStep < 3 ? 'Next' : 'Done'),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
