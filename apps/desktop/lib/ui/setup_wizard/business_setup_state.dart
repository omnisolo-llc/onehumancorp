import 'package:flutter_riverpod/flutter_riverpod.dart';

class BusinessSetupState {
  final int step;
  final String businessType;
  final String companyName;

  BusinessSetupState({
    this.step = 0,
    this.businessType = '',
    this.companyName = '',
  });

  BusinessSetupState copyWith({
    int? step,
    String? businessType,
    String? companyName,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      businessType: businessType ?? this.businessType,
      companyName: companyName ?? this.companyName,
    );
  }
}

class BusinessSetupNotifier extends StateNotifier<BusinessSetupState> {
  BusinessSetupNotifier() : super(BusinessSetupState());

  void setStep(int step) {
    state = state.copyWith(step: step);
  }

  void setBusinessType(String type) {
    state = state.copyWith(businessType: type);
  }

  void setCompanyName(String name) {
    state = state.copyWith(companyName: name);
  }

  void nextStep() {
    state = state.copyWith(step: state.step + 1);
  }

  void previousStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }
}

final businessSetupProvider = StateNotifierProvider<BusinessSetupNotifier, BusinessSetupState>((ref) {
  return BusinessSetupNotifier();
});
