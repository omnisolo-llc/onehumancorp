import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  group('BusinessSetupNotifier', () {
    testWidgets('initial state is correct', (tester) async {
      final container = ProviderContainer();
      final state = container.read(businessSetupProvider);

      expect(state.step, 0);
      expect(state.companyName, '');
      expect(state.industry, '');
      expect(state.size, 'S');
      expect(state.goals, isEmpty);
      expect(state.deployment, 'Cloud');
      expect(state.adminName, '');
      expect(state.adminEmail, '');
      expect(state.adminPassword, '');
      expect(state.isLoading, false);
      expect(state.errorMessage, isNull);
    });

    testWidgets('updateCompany changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(businessSetupProvider.notifier);

      notifier.updateCompany('Test Corp');
      expect(container.read(businessSetupProvider).companyName, 'Test Corp');
    });

    testWidgets('updateIndustry changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(businessSetupProvider.notifier);

      notifier.updateIndustry('Tech');
      expect(container.read(businessSetupProvider).industry, 'Tech');
    });

    testWidgets('updateSize changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(businessSetupProvider.notifier);

      notifier.updateSize('L');
      expect(container.read(businessSetupProvider).size, 'L');
    });

    testWidgets('toggleGoal changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(businessSetupProvider.notifier);

      notifier.toggleGoal('Support');
      expect(container.read(businessSetupProvider).goals, contains('Support'));

      notifier.toggleGoal('Support');
      expect(container.read(businessSetupProvider).goals, isEmpty);
    });

    testWidgets('updateDeployment changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(businessSetupProvider.notifier);

      notifier.updateDeployment('Desktop');
      expect(container.read(businessSetupProvider).deployment, 'Desktop');
    });

    testWidgets('updateAdmin updates admin state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(businessSetupProvider.notifier);

      notifier.updateAdminName('Admin');
      notifier.updateAdminEmail('admin@test.local');
      notifier.updateAdminPassword('password');

      expect(container.read(businessSetupProvider).adminName, 'Admin');
      expect(container.read(businessSetupProvider).adminEmail, 'admin@test.local');
      expect(container.read(businessSetupProvider).adminPassword, 'password');
    });

    testWidgets('nextStep and prevStep manage state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(businessSetupProvider.notifier);

      expect(container.read(businessSetupProvider).step, 0);

      notifier.nextStep();
      expect(container.read(businessSetupProvider).step, 1);

      for (int i = 0; i < 5; i++) {
         notifier.nextStep();
      }
      expect(container.read(businessSetupProvider).step, 4); // Max step

      notifier.prevStep();
      expect(container.read(businessSetupProvider).step, 3);

      for (int i = 0; i < 5; i++) {
         notifier.prevStep();
      }
      expect(container.read(businessSetupProvider).step, 0); // Min step
    });
  });
}