import re

with open("srcs/app/lib/screens/referrals_dashboard_screen.dart", "r") as f:
    content = f.read()

content = re.sub(
    r'(late Future<List<Map<String, dynamic>>> _referralsFuture;)',
    r'\1\n  late Future<Map<String, dynamic>> _viralCoefficientFuture;\n  late Future<List<dynamic>> _combinedFuture;',
    content
)

content = re.sub(
    r'(_referralsFuture = ref\.read\(apiServiceProvider\)!.*?listReferrals\(\);)',
    r'''\1
      _viralCoefficientFuture = ref.read(apiServiceProvider)!.getViralCoefficient();
      _combinedFuture = Future.wait([_referralsFuture, _viralCoefficientFuture]);''',
    content
)

content = re.sub(
    r'(body: FutureBuilder<List<Map<String, dynamic>>>\(\s+future: _referralsFuture,\s+builder: \(context, snapshot\) \{\s+if \(snapshot\.connectionState == ConnectionState\.waiting\) \{\s+return const Center\(child: CircularProgressIndicator\(\)\);\s+\}\s+if \(snapshot\.hasError\) \{\s+return Center\(\s+child: Text\(\s+\'Error: \$\{snapshot\.error\}\',\s+style: TextStyle\(color: colors\.error\),\s+\),\s+\);\s+\}\s+final referrals = snapshot\.data \?\? \[\];\s+)',
    r'''body: FutureBuilder<List<dynamic>>(
        future: _combinedFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          }
          if (snapshot.hasError) {
            return Center(
              child: Text(
                'Error: ${snapshot.error}',
                style: TextStyle(color: colors.error),
              ),
            );
          }
          final referrals = snapshot.data?[0] as List<Map<String, dynamic>>? ?? [];
          final vcData = snapshot.data?[1] as Map<String, dynamic>? ?? {};
          final kFactor = vcData['kFactor'] ?? 0.0;
          return Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Padding(
                padding: const EdgeInsets.all(24),
                child: GlassCard(
                  padding: const EdgeInsets.all(24),
                  color: colors.surface.withValues(alpha: 0.6),
                  child: Column(
                    children: [
                      const Text(
                        'Viral Coefficient (K-Factor)',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 16),
                      Text(
                        kFactor.toStringAsFixed(2),
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 48,
                          fontWeight: FontWeight.bold,
                          color: colors.primary,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              Expanded(
                child: Builder(
              builder: (context) {
                ''',
    content
)

content = re.sub(
    r'(if \(referrals\.isEmpty\) \{)',
    r'\1',
    content
)

content = re.sub(
    r'(\}\)\.toList\(\),\s+\),\s+\);\s+\},\s+\),)',
    r'''}).toList(),
            ),
          );
              },
            ),
          ),
        ],
      );
    },
  ),''',
    content
)

with open("srcs/app/lib/screens/referrals_dashboard_screen.dart", "w") as f:
    f.write(content)
