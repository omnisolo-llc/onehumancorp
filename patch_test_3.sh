sed -i 's/import .package:flutter_riverpod\/flutter_riverpod.dart.;/import '\''package:flutter_riverpod\/flutter_riverpod.dart'\'';\nimport '\''package:shared_preferences\/shared_preferences.dart'\'';/' srcs/app/test/desktop_e2e_test.dart
sed -i 's/void main() {/void main() {\n  SharedPreferences.setMockInitialValues({});/' srcs/app/test/desktop_e2e_test.dart
