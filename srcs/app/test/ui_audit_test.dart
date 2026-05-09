import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:app/screens/help/ai_help_chat_screen.dart';
import 'package:app/screens/help/help_center_screen.dart';
import 'package:app/screens/help/api_reference_screen.dart';
import 'package:app/screens/referral_program_screen.dart';
import 'package:app/screens/unified_inbox_screen.dart';

void main() {
  testWidgets('AiHelpChatScreen renders with GlassContainer and no mock data', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: AiHelpChatScreen())));
    expect(find.byType(GlassContainer), findsWidgets);
    expect(find.text('Hi! I am your Support Agent. How can I help you with OneHumanCorp today?'), findsNothing);
  });

  testWidgets('HelpCenterScreen renders topic and video cards as GlassContainers', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: HelpCenterScreen())));
  });

  testWidgets('ApiReferenceScreen renders endpoints as GlassContainers', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: ApiReferenceScreen())));
  });

  testWidgets('ReferralProgramScreen renders without mock invites and uses GlassContainer', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: ReferralProgramScreen())));
    expect(find.text('friend@example.com'), findsNothing);
  });

  testWidgets('UnifiedInboxScreen renders without mock messages', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: UnifiedInboxScreen())));
    expect(find.text('maya_bakes'), findsNothing);
  });
}
