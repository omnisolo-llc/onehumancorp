import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  test('BusinessSetupNotifier covers all state mutations', () {
    final container = ProviderContainer();
    final notifier = container.read(businessSetupProvider.notifier);

    notifier.updateBusinessType('Retail');
    expect(container.read(businessSetupProvider).businessType, 'Retail');

    notifier.updateCompany('Test Co');
    expect(container.read(businessSetupProvider).companyName, 'Test Co');

    notifier.updateDescription('Test Desc');
    expect(container.read(businessSetupProvider).description, 'Test Desc');

    notifier.toggleWhatYouSell('Physical products');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Physical products'), true);

    notifier.toggleWhatYouSell('Physical products');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Physical products'), false);

    notifier.updatePayments('Online only');
    expect(container.read(businessSetupProvider).payments, 'Online only');

    notifier.toggleObscurePassword();
    expect(container.read(businessSetupProvider).obscurePassword, false);
  });
}
