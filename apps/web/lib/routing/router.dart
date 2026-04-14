import 'package:flutter/material.dart';
import '../screens/prompt_tuning_wizard_screen.dart';

class AppRouter {
  static Route<dynamic> generateRoute(RouteSettings settings) {
    switch (settings.name) {
      case '/prompt-tuning':
        return MaterialPageRoute(builder: (_) => const PromptTuningWizardScreen());
      default:
        return MaterialPageRoute(
          builder: (_) => Scaffold(
            body: Center(
              child: Text('No route defined for ${settings.name}'),
            ),
          ),
        );
    }
  }
}
