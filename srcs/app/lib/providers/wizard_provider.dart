import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

class WizardState {
  final int currentStep;
  final String? companyName;
  final String? industry;
  final String? primaryGoal;
  final String? templateSelection;
  final String? paymentSetupMode;

  WizardState({
    this.currentStep = 0,
    this.companyName,
    this.industry,
    this.primaryGoal,
    this.templateSelection,
    this.paymentSetupMode,
  });

  WizardState copyWith({
    int? currentStep,
    String? companyName,
    String? industry,
    String? primaryGoal,
    String? templateSelection,
    String? paymentSetupMode,
  }) {
    return WizardState(
      currentStep: currentStep ?? this.currentStep,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      primaryGoal: primaryGoal ?? this.primaryGoal,
      templateSelection: templateSelection ?? this.templateSelection,
      paymentSetupMode: paymentSetupMode ?? this.paymentSetupMode,
    );
  }
}

class WizardNotifier extends Notifier<WizardState> {
  final ApiService _apiService = ApiService();

  @override
  WizardState build() {
    return WizardState();
  }

  void nextStep() {
    if (state.currentStep < 6) {
      state = state.copyWith(currentStep: state.currentStep + 1);
    }
  }

  void prevStep() {
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
    }
  }

  void updateBusinessProfile({String? companyName, String? industry}) {
    state = state.copyWith(
      companyName: companyName ?? state.companyName,
      industry: industry ?? state.industry,
    );
  }

  void setPrimaryGoal(String goal) {
    state = state.copyWith(primaryGoal: goal);
  }

  void setTemplateSelection(String template) {
    state = state.copyWith(templateSelection: template);
  }

  void setPaymentSetupMode(String mode) {
    state = state.copyWith(paymentSetupMode: mode);
  }

  Future<void> submitWizard() async {
    final data = {
      'companyName': state.companyName,
      'industry': state.industry,
      'primaryGoal': state.primaryGoal,
      'templateSelection': state.templateSelection,
      'paymentSetupMode': state.paymentSetupMode,
    };

    await _apiService.submitBusinessData(data);

    // Proceed to the dashboard
    nextStep();
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
