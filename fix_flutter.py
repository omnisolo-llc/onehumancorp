import re

with open("srcs/app/lib/screens/referrals_dashboard_screen.dart", "r") as f:
    content = f.read()

content = re.sub(
    r'(void _refresh\(\) \{\n\s+setState\(\(\) \{\n\s+_referralsFuture = ref\.read\(apiServiceProvider\)!.*?listReferrals\(\);\n\s+_viralCoefficientFuture = ref\.read\(apiServiceProvider\)!.*?getViralCoefficient\(\);\n\s+\}\);\n\s+_combinedFuture = Future\.wait\(\[_referralsFuture, _viralCoefficientFuture\]\);\n\s+\}\);\n\s+\})',
    r'''void _refresh() {
    setState(() {
      _referralsFuture = ref.read(apiServiceProvider)!.listReferrals();
      _viralCoefficientFuture = ref.read(apiServiceProvider)!.getViralCoefficient();
      _combinedFuture = Future.wait([_referralsFuture, _viralCoefficientFuture]);
    });
  }''',
    content
)

# And fix the ending braces
content = re.sub(
    r'(\}\)\.toList\(\),\n\s+\),\n\s+\);\n\s+\},\n\s+\),\n\s+\),\n\s+\],\n\s+\);\n\s+\},\n\s+\),\n\s+\);\n\s+\}\n\})',
    r'''}).toList(),
            ),
          );
              },
            ),
          ),
        ],
      );
    },
  ),
    );
  }
}''',
    content
)


with open("srcs/app/lib/screens/referrals_dashboard_screen.dart", "w") as f:
    f.write(content)
