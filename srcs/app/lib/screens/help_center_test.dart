import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApi;

  setUp(() {
    mockApi = MockApiService();
    registerFallbackValue(Uri.parse('http://localhost'));
  });

  Widget createTestWidget() {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(mockApi),
        authStateProvider.overrideWith(() => AuthNotifierStub(const AuthUser(id: '1', email: 'admin@ohc.com', name: 'Admin', role: 'admin', organizationId: 'org-1', token: 'test'))),
      ],
      child: const MaterialApp(
        home: HelpCenterScreen(),
      ),
    );
  }

  testWidgets('HelpCenterScreen displays articles', (WidgetTester tester) async {
    when(() => mockApi.listHelpArticles(query: any(named: 'query')))
        .thenAnswer((_) async => [
              {'title': 'Test Article', 'topic': 'General', 'content': 'Test Content'}
            ]);
    when(() => mockApi.listHelpVideos()).thenAnswer((_) async => []);

    await tester.pumpWidget(createTestWidget());
    await tester.pumpAndSettle();

    expect(find.text('Test Article'), findsOneWidget);
    expect(find.text('General'), findsOneWidget);

    await tester.tap(find.text('Test Article'));
    await tester.pumpAndSettle();

    expect(find.text('Test Content'), findsOneWidget);
  });
}

class AuthNotifierStub extends AuthNotifier {
  final AuthUser? user;
  AuthNotifierStub(this.user);

  @override
  Future<AuthUser?> build() async => user;
}
