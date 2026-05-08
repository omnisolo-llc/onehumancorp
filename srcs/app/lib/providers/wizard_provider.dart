import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

class WizardState {
  final int currentStep;
  final String? companyName;
  final String? industry;
  final String? size;
  final List<String> goals;
  final String? deploymentPreference;
  final String? adminName;
  final String? adminEmail;
  final String? adminPassword;

  final bool sellPhysical;
  final bool sellDigital;
  final bool sellServices;
  final bool sellFood;
  final bool sellSubscriptions;

  final String? paymentPreference;
  final String? websiteTemplate;

  final String? productName;
  final String? productDescription;
  final String? productPrice;

  final String? domainChoice;
  final bool launchSuccess;

  WizardState({
    this.currentStep = 0,
    this.companyName,
    this.industry,
    this.size,
    this.goals = const [],
    this.deploymentPreference,
    this.adminName,
    this.adminEmail,
    this.adminPassword,
    this.sellPhysical = false,
    this.sellDigital = false,
    this.sellServices = false,
    this.sellFood = false,
    this.sellSubscriptions = false,
    this.paymentPreference,
    this.websiteTemplate,
    this.productName,
    this.productDescription,
    this.productPrice,
    this.domainChoice,
    this.launchSuccess = false,
  });

  WizardState copyWith({
    int? currentStep,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? deploymentPreference,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    bool? sellPhysical,
    bool? sellDigital,
    bool? sellServices,
    bool? sellFood,
    bool? sellSubscriptions,
    String? paymentPreference,
    String? websiteTemplate,
    String? productName,
    String? productDescription,
    String? productPrice,
    String? domainChoice,
    bool? launchSuccess,
  }) {
    return WizardState(
      currentStep: currentStep ?? this.currentStep,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      deploymentPreference: deploymentPreference ?? this.deploymentPreference,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      sellPhysical: sellPhysical ?? this.sellPhysical,
      sellDigital: sellDigital ?? this.sellDigital,
      sellServices: sellServices ?? this.sellServices,
      sellFood: sellFood ?? this.sellFood,
      sellSubscriptions: sellSubscriptions ?? this.sellSubscriptions,
      paymentPreference: paymentPreference ?? this.paymentPreference,
      websiteTemplate: websiteTemplate ?? this.websiteTemplate,
      productName: productName ?? this.productName,
      productDescription: productDescription ?? this.productDescription,
      productPrice: productPrice ?? this.productPrice,
      domainChoice: domainChoice ?? this.domainChoice,
      launchSuccess: launchSuccess ?? this.launchSuccess,
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
    state = state.copyWith(currentStep: state.currentStep + 1);
  }

  void prevStep() {
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
    }
  }

  void updateBusinessProfile({String? companyName, String? industry, String? size}) {
    state = state.copyWith(
      companyName: companyName ?? state.companyName,
      industry: industry ?? state.industry,
      size: size ?? state.size,
    );
  }

  void toggleGoal(String goal) {
    final currentGoals = List<String>.from(state.goals);
    if (currentGoals.contains(goal)) {
      currentGoals.remove(goal);
    } else {
      currentGoals.add(goal);
    }
    state = state.copyWith(goals: currentGoals);
  }

  void setDeploymentPreference(String preference) {
    state = state.copyWith(deploymentPreference: preference);
  }

  void updateAdminAccount({String? name, String? email, String? password}) {
    state = state.copyWith(
      adminName: name ?? state.adminName,
      adminEmail: email ?? state.adminEmail,
      adminPassword: password ?? state.adminPassword,
    );
  }

  void setPaymentPreference(String preference) {
    state = state.copyWith(paymentPreference: preference);
  }

  void toggleSellingCategory({
    bool? physical,
    bool? digital,
    bool? services,
    bool? food,
    bool? subscriptions,
  }) {
    state = state.copyWith(
      sellPhysical: physical ?? state.sellPhysical,
      sellDigital: digital ?? state.sellDigital,
      sellServices: services ?? state.sellServices,
      sellFood: food ?? state.sellFood,
      sellSubscriptions: subscriptions ?? state.sellSubscriptions,
    );
  }

  void setWebsiteTemplate(String template) {
    state = state.copyWith(websiteTemplate: template);
  }

  void setProductDetails({String? name, String? description, String? price}) {
    state = state.copyWith(
      productName: name ?? state.productName,
      productDescription: description ?? state.productDescription,
      productPrice: price ?? state.productPrice,
    );
  }

  void setDomainChoice(String choice) {
    state = state.copyWith(domainChoice: choice);
  }

  void markLaunchSuccess() {
    state = state.copyWith(launchSuccess: true);
  }

  void reset() {
    state = WizardState();
  }

  Future<void> submitWizard() async {
    final data = {
      'companyName': state.companyName,
      'industry': state.industry,
      'size': state.size,
      'goals': state.goals,
      'deploymentPreference': state.deploymentPreference,
      'adminName': state.adminName,
      'adminEmail': state.adminEmail,
      'adminPassword': state.adminPassword,
      'sellPhysical': state.sellPhysical,
      'sellDigital': state.sellDigital,
      'sellServices': state.sellServices,
      'sellFood': state.sellFood,
      'sellSubscriptions': state.sellSubscriptions,
      'paymentPreference': state.paymentPreference,
      'websiteTemplate': state.websiteTemplate,
      'productName': state.productName,
      'productDescription': state.productDescription,
      'productPrice': state.productPrice,
      'domainChoice': state.domainChoice,
    };

    await _apiService.submitBusinessData(data);
    markLaunchSuccess();
    // Proceed to checklist/dashboard handled by UI directly
  }
}

final wizardProvider = NotifierProvider<WizardNotifier, WizardState>(WizardNotifier.new);
