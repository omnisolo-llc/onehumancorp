import re

def rewrite(filepath, port):
    with open(filepath, 'r') as f:
        content = f.read()

    # 1. Add dart:io
    if "import 'dart:io';" not in content:
        content = "import 'dart:io';\n" + content

    server_block = f"""
  HttpServer? _testServer;

  setUpAll(() async {{
    registerFallbackValue(FakeUri());
    SharedPreferences.setMockInitialValues({{}});

    _testServer = await HttpServer.bind(InternetAddress.loopbackIPv4, {port});
    _testServer!.listen((HttpRequest request) {{
      request.response.headers.contentType = ContentType.json;
      request.response.headers.add('Access-Control-Allow-Origin', '*');

      final path = request.uri.path;
      if (path.contains('/api/meetings')) {{
        if (request.method == 'POST') {{
          request.response.write('{{"id": "m1", "name": "launch-readiness"}}');
        }} else {{
          request.response.write('[{{\"id\": \"m1\", \"name\": \"launch-readiness\", \"participants\": []}}]');
        }}
      }} else if (path.contains('/api/agents/providers')) {{
         request.response.write('[{{\"id\": \"p1\", \"name\": \"gpt-4o-mini\"}}]');
      }} else if (path.contains('/api/agents/hire')) {{
         request.response.write('{{\"id\": \"a2\", \"name\": \"New Agent\"}}');
      }} else if (path.contains('/api/agents/fire')) {{
         request.response.write('{{}}');
      }} else if (path.contains('/api/agents')) {{
         request.response.write('[{{\"id\": \"a1\", \"name\": \"Software Engineer\"}}]');
      }} else if (path.contains('/api/ai/providers')) {{
         if (request.method == 'POST' || request.method == 'PATCH') {{
             request.response.write('{{\"id\": \"p1\", \"name\": \"gpt-4o-mini\", \"base_url\": \"https://api.openai.com/v1\", \"api_key\": \"sk-...\", \"models\": [\"gpt-4o-mini\"], \"is_official\": true}}');
         }} else {{
             request.response.write('[{{\"id\": \"p1\", \"name\": \"gpt-4o-mini\", \"base_url\": \"https://api.openai.com/v1\", \"api_key\": \"sk-...\", \"models\": [\"gpt-4o-mini\"], \"is_official\": true}}]');
         }}
      }} else if (path.contains('/api/providers')) {{
         request.response.write('[{{\"id\": \"p1\", \"name\": \"gpt-4o-mini\", \"base_url\": \"https://api.openai.com/v1\", \"api_key\": \"sk-...\", \"models\": [\"gpt-4o-mini\"], \"is_official\": true}}]');
      }} else if (path.contains('/api/skills')) {{
         request.response.write('[{{\"name\": \"web_search\", \"version\": \"1.0.0\", \"description\": \"Search the web\", \"category\": \"official\", \"installed\": true, \"enabled\": true}}]');
      }} else if (path.contains('/api/channels')) {{
         if (request.method == 'POST') {{
             request.response.write('{{\"id\": \"c1\", \"name\": \"general\", \"type\": \"public\"}}');
         }} else {{
             request.response.write('[]');
         }}
      }} else if (path.contains('/auth/login')) {{
         request.response.write('{{"token": "new-token", "user": {{"id": "u1", "email": "a@b.com", "name": "Alice", "role": "admin", "organization_id": "org-1"}}}}');
      }} else {{
        request.response.write('[]');
      }}
      request.response.close();
    }});
  }});

  tearDownAll(() async {{
    await _testServer?.close(force: true);
  }});
"""

    # 2. Re-route MockHttpClient
    # Instead of deleting MockHttpClient, we keep it as a mock for tests that EXPLICITLY verify network calls, BUT we inject our local server into `ApiService` for all tests.

    # Wait, if we keep MockHttpClient, any test that overrides apiService will use `http://127.0.0.1:{port}`.

    # Let's replace `ApiService _mockApi(MockHttpClient client) =>` with `ApiService _mockApi(MockHttpClient client) => ApiService(baseUrl: 'http://127.0.0.1:{port}', token: 'tok');`
    content = re.sub(
        r"ApiService _mockApi\(MockHttpClient client\) =>[\s\S]*?token: 'tok'\);",
        f"ApiService _mockApi(MockHttpClient client) =>\n    ApiService(baseUrl: 'http://127.0.0.1:{port}', token: 'tok');",
        content
    )

    # Then insert the HttpServer logic.
    if "HttpServer? _testServer" not in content:
        content = content.replace("void main() {", f"void main() {{\n{server_block}", 1)

    # 3. Add pump delays
    content = re.sub(
        r"(await tester\.pumpWidget\(.*?\);\s*)await tester\.pumpAndSettle\(\);",
        r"\1await tester.pumpAndSettle();\n      await tester.pump(const Duration(milliseconds: 100));\n      await tester.pumpAndSettle();",
        content,
        flags=re.DOTALL
    )

    if filepath.endswith("advanced_widget_test.dart"):
        content = content.replace("class FakeClientConfig extends Fake implements centrifuge.ClientConfig {}", "class FakeClientConfig extends Fake implements centrifuge.ClientConfig {}")
        content = content.replace("registerFallbackValue(FakeUri());", "registerFallbackValue(FakeUri());\n    registerFallbackValue(FakeClientConfig());")

    with open(filepath, 'w') as f:
        f.write(content)

rewrite('srcs/app/lib/screens/widget_interactions_test.dart', 8083)
rewrite('srcs/app/lib/screens/advanced_widget_test.dart', 8082)
