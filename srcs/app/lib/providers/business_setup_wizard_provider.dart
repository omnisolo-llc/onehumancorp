import 'package:flutter_riverpod/flutter_riverpod.dart';

class BusinessSetupState {
  final int step;
  final String companyName;
  final String industry;
  final String size;
  final List<String> goals;
  final String deployment;
  final String adminName;
  final String adminEmail;
  final String adminPassword;

  const BusinessSetupState({
    this.step = 0,
    this.companyName = '',
    this.industry = '',
    this.size = '1-10',
    this.goals = const [],
    this.deployment = 'Cloud',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
  });

  BusinessSetupState copyWith({
    int? step,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? deployment,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      deployment: deployment ?? this.deployment,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
    );
  }
}

class BusinessSetupWizardNotifier extends StateNotifier<BusinessSetupState> {
  BusinessSetupWizardNotifier() : super(const BusinessSetupState());

  void nextStep() {
    if (state.step < 5) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void previousStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void setCompanyName(String name) => state = state.copyWith(companyName: name);
  void setIndustry(String industry) => state = state.copyWith(industry: industry);
  void setSize(String size) => state = state.copyWith(size: size);

  void addGoal(String goal) {
    if (!state.goals.contains(goal)) {
      state = state.copyWith(goals: [...state.goals, goal]);
    }
  }

  void removeGoal(String goal) {
    state = state.copyWith(goals: state.goals.where((g) => g != goal).toList());
  }

  void setDeployment(String deployment) => state = state.copyWith(deployment: deployment);
  void setAdminName(String name) => state = state.copyWith(adminName: name);
  void setAdminEmail(String email) => state = state.copyWith(adminEmail: email);
  void setAdminPassword(String password) => state = state.copyWith(adminPassword: password);
}

final businessSetupWizardProvider = StateNotifierProvider<BusinessSetupWizardNotifier, BusinessSetupState>((ref) {
  return BusinessSetupWizardNotifier();
});
