import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/grow_my_business_wizard_screen.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';
import 'package:ohc_app/screens/agent_config_wizard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends Mock implements ApiService {}
class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, ApiService api) {
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: MaterialApp(
      home: screen,
    ),
  );
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('Ongoing Management Wizards CUJ E2E', () {
    late MockApiService mockApi;

    setUp(() {
      mockApi = MockApiService();
    });

    testWidgets('Grow My Business Wizard basic flow', (tester) async {
      tester.view.physicalSize = const Size(1920, 1080);
      tester.view.devicePixelRatio = 1.0;
      await tester.pumpWidget(_wrapScreen(const GrowMyBusinessWizardScreen(), mockApi));
      await tester.pump(const Duration(milliseconds: 500));

      expect(find.textContaining('Growth Strategies'), findsWidgets);

      final nextButton = find.text('Next');
      expect(nextButton, findsWidgets);
      await tester.tap(nextButton.first);
      await tester.pump(const Duration(milliseconds: 500));
    });

    testWidgets('Website Builder Wizard basic flow', (tester) async {
      tester.view.physicalSize = const Size(1920, 1080);
      tester.view.devicePixelRatio = 1.0;
      await tester.pumpWidget(_wrapScreen(const WebsiteBuilderWizardScreen(), mockApi));
      await tester.pump(const Duration(milliseconds: 500));

      expect(find.textContaining('Website Builder'), findsWidgets);

      final nextButton = find.text('Next');
      expect(nextButton, findsWidgets);
      await tester.tap(nextButton.first);
      await tester.pump(const Duration(milliseconds: 500));
    });

    testWidgets('Configure AI Agents Wizard basic flow', (tester) async {
      tester.view.physicalSize = const Size(1920, 1080);
      tester.view.devicePixelRatio = 1.0;
      await tester.pumpWidget(_wrapScreen(const AgentConfigWizardScreen(), mockApi));
      await tester.pump(const Duration(milliseconds: 500));

      expect(find.textContaining('Configure AI Agent'), findsWidgets);

      await tester.tap(find.text('Customer Support').first);
      await tester.pump(const Duration(milliseconds: 500));

      final nextButton = find.text('Next');
      expect(nextButton, findsWidgets);
      await tester.tap(nextButton.first);
      await tester.pump(const Duration(milliseconds: 500));
    });
  });
}
