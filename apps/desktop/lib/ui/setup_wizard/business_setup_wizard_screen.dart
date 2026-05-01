import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'business_setup_state.dart';

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  late TextEditingController _businessTypeController;
  late TextEditingController _companyNameController;

  @override
  void initState() {
    super.initState();
    final state = ref.read(businessSetupProvider);
    _businessTypeController = TextEditingController(text: state.businessType);
    _companyNameController = TextEditingController(text: state.companyName);
  }

  @override
  void dispose() {
    _businessTypeController.dispose();
    _companyNameController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    // Update controllers if state changed externally
    if (_businessTypeController.text != state.businessType &&
        !FocusScope.of(context).hasFocus) {
      _businessTypeController.text = state.businessType;
    }
    if (_companyNameController.text != state.companyName &&
        !FocusScope.of(context).hasFocus) {
      _companyNameController.text = state.companyName;
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Setup Wizard'),
        leading: state.step > 0
          ? IconButton(
              icon: const Icon(Icons.arrow_back),
              onPressed: notifier.previousStep,
            )
          : null,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Text('Step: ${state.step}'),
            if (state.step == 0) ...[
              const Text('Welcome'),
              const Text('Welcome to Business Setup'),
            ],
            if (state.step == 1) ...[
              const Text('Business Type'),
              ElevatedButton(
                onPressed: () {
                  notifier.setBusinessType('Online Store');
                  _businessTypeController.text = 'Online Store';
                },
                child: const Text('Online Store'),
              ),
              ElevatedButton(
                onPressed: () {
                  notifier.setBusinessType('Service Business');
                  _businessTypeController.text = 'Service Business';
                },
                child: const Text('Service Business'),
              ),
              ElevatedButton(
                onPressed: () {
                  notifier.setBusinessType('Restaurant');
                  _businessTypeController.text = 'Restaurant';
                },
                child: const Text('Restaurant'),
              ),
              ElevatedButton(
                onPressed: () {
                  notifier.setBusinessType('Creative');
                  _businessTypeController.text = 'Creative';
                },
                child: const Text('Creative'),
              ),
              ElevatedButton(
                onPressed: () {
                  notifier.setBusinessType('Local Business');
                  _businessTypeController.text = 'Local Business';
                },
                child: const Text('Local Business'),
              ),
              TextField(
                decoration: const InputDecoration(labelText: 'Custom Business Type'),
                onChanged: notifier.setBusinessType,
                controller: _businessTypeController,
              ),
            ],
            if (state.step == 2) ...[
              TextField(
                decoration: const InputDecoration(labelText: 'Company Name'),
                onChanged: notifier.setCompanyName,
                controller: _companyNameController,
              ),
            ],
            if (state.step == 3) ...[
              const Text('What do you sell'),
              const Text('Physical'),
              const Text('Digital'),
              const Text('Services'),
            ],
            if (state.step == 4) ...[
              const Text('Payment'),
            ],
            if (state.step == 5) ...[
              const Text('Admin Account'),
              const TextField(
                decoration: InputDecoration(labelText: 'Email'),
                keyboardType: TextInputType.emailAddress,
              ),
              const TextField(
                decoration: InputDecoration(labelText: 'Password'),
                obscureText: true,
              ),
            ],
            if (state.step == 6) ...[
              const Text('Template'),
            ],
            if (state.step == 7) ...[
              const Text('Domain'),
            ],
            if (state.step == 8) ...[
              const Text('Review and Launch'),
              Text('Type: ${state.businessType}'),
              Text('Name: ${state.companyName}'),
              ElevatedButton(
                onPressed: () {},
                child: const Text('Launch'),
              ),
            ],
            const Spacer(),
            if (state.step < 8)
              ElevatedButton(
                onPressed: notifier.nextStep,
                child: const Text('Next'),
              ),
            if (state.step > 0)
              ElevatedButton(
                onPressed: notifier.previousStep,
                child: const Text('Back'),
              ),
          ],
        ),
      ),
    );
  }
}
