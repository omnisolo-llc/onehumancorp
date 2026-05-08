import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';
import '../services/telemetry_service.dart';

class WizardState {
  final int currentStep;
  final String? companyName;
  final String? category;
  final bool isLoading;
  final String? errorMessage;

  WizardState({
    this.currentStep = 0,
    this.companyName,
    this.category,
    this.isLoading = false,
    this.errorMessage,
  });

  WizardState copyWith({
    int? currentStep,
    String? companyName,
    String? category,
    bool? isLoading,
    String? errorMessage,
  }) {
    return WizardState(
      currentStep: currentStep ?? this.currentStep,
      companyName: companyName ?? this.companyName,
      category: category ?? this.category,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }

  WizardState clearError() {
      return WizardState(
        currentStep: currentStep,
        companyName: companyName,
        category: category,
        isLoading: isLoading,
        errorMessage: null,
      );
  }
}

class WizardNotifier extends Notifier<WizardState> {
  final ApiService _apiService = ApiService();
  final TelemetryService _telemetryService = TelemetryService();

  @override
  WizardState build() {
    _telemetryService.trackEvent('onboarding_started');
    return WizardState();
  }

  void nextStep() {
    state = state.clearError();
    if (state.currentStep < 2) {
      state = state.copyWith(currentStep: state.currentStep + 1);
      _telemetryService.trackEvent('onboarding_step_completed', properties: {'step': state.currentStep});
    }
  }

  void prevStep() {
    state = state.clearError();
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
    }
  }

  void setCategory(String category) {
    state = state.copyWith(category: category);
  }

  void setCompanyName(String name) {
    state = state.copyWith(companyName: name);
  }

  Future<void> submitWizard() async {
    state = state.clearError().copyWith(isLoading: true);
    _telemetryService.trackEvent('onboarding_submission_started');

    try {
      final data = {
        'companyName': state.companyName,
        'category': state.category,
      };

      await _apiService.submitBusinessData(data);

      _telemetryService.trackEvent('onboarding_submission_success');
      state = state.copyWith(isLoading: false, currentStep: 3); // Move to dashboard (step 3)
    } catch (e) {
      _telemetryService.trackError('onboarding_submission_failed', e.toString());
      state = state.clearError().copyWith(
        isLoading: false,
        errorMessage: 'Network error. Please check your connection and try again.',
      );
    }
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
