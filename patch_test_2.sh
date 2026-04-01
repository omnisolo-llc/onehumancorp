sed -i 's/await tester.pumpAndSettle();/await tester.pumpAndSettle(); await tester.pump();/g' srcs/app/test/desktop_e2e_test.dart
