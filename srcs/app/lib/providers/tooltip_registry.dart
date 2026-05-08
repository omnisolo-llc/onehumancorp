import 'package:flutter_riverpod/flutter_riverpod.dart';

final tooltipRegistryProvider = Provider<Map<String, String>>((ref) {
  return {
    'industryDropdown': 'Select the category that best describes your business to help us tailor your AI team.',
    'sizeDropdown': 'The number of employees helps us configure the right scale for your deployment.',
    'goalsBuildSoftware': 'Choose this if you want to create your own digital products or SaaS.',
    'goalsSupport': 'Choose this if you need an AI agent to answer customer questions 24/7.',
    'deploymentCloud': 'Cloud is the easiest option. We host everything for you securely.',
    'deploymentStandalone': 'Standalone gives you full control and runs entirely on your own hardware.',
  };
});
