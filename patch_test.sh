#!/bin/bash
sed -i 's/await tester.pump();/await tester.pump(const Duration(seconds: 1));/g' srcs/app/test/desktop_e2e_test.dart
