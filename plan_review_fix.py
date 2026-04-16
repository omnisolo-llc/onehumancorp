import re

# Fix 1: Initialize viralLoopTracker in NewServer (server.go)
with open("srcs/server/dashboard/server.go", "r") as f:
    content = f.read()

# I already tried this but maybe I missed it? Let's check NewServer.
# Wait, I did `viralLoopTracker: growth.NewViralLoopTracker(nil)` but it might be missing or wrong place.
content = re.sub(
    r'(waitlist:\s+\[\]WaitlistEntry\{\},)',
    r'\1\n\t\tviralLoopTracker: growth.NewViralLoopTracker(nil),',
    content
)

with open("srcs/server/dashboard/server.go", "w") as f:
    f.write(content)

# Fix 2: handlers_growth.go - Move s.viralLoopTracker.RecordInviteAccepted outside the break inside loop
with open("srcs/server/dashboard/handlers_growth.go", "r") as f:
    content = f.read()

# Let's fix handleTeamInviteAccept
# Currently it looks like:
# for i, inv := range s.teamInvites {
#   if inv.ID == req.ID {
#     ...
#     found = true
#     break
#   }
#   if found && s.viralLoopTracker != nil { ... }
# }
# Wait, the block:
# 	if found && s.viralLoopTracker != nil {
# 		s.viralLoopTracker.RecordInviteAccepted(r.Context(), updated.InviteeID)
# 	}
# It was inside the loop. Let's move it out.
content = re.sub(
    r'(\n\n\t*if found && s\.viralLoopTracker != nil \{\n\t\t*s\.viralLoopTracker\.RecordInviteAccepted\(r\.Context\(\), updated\.InviteeID\)\n\t*\})',
    r'',
    content
)

# Put it back right after the loop.
content = re.sub(
    r'(break\n\t\t\}\n\t\})',
    r'\1\n\n\tif found && s.viralLoopTracker != nil {\n\t\ts.viralLoopTracker.RecordInviteAccepted(r.Context(), updated.InviteeID)\n\t}',
    content
)

with open("srcs/server/dashboard/handlers_growth.go", "w") as f:
    f.write(content)


# Fix 3: Flutter future in initState instead of build
with open("srcs/app/lib/screens/referrals_dashboard_screen.dart", "r") as f:
    content = f.read()

content = re.sub(
    r'(late Future<List<Map<String, dynamic>>> _referralsFuture;\n\s+late Future<Map<String, dynamic>> _viralCoefficientFuture;)',
    r'\1\n  late Future<List<dynamic>> _combinedFuture;',
    content
)

content = re.sub(
    r'(_referralsFuture = ref\.read\(apiServiceProvider\)!.*?listReferrals\(\);\n\s+_viralCoefficientFuture = ref\.read\(apiServiceProvider\)!.*?getViralCoefficient\(\);\n\s+\}\);)',
    r'''\1
      _combinedFuture = Future.wait([_referralsFuture, _viralCoefficientFuture]);
    });''',
    content
)

content = re.sub(
    r'(body: FutureBuilder<List<dynamic>>\(\n\s+future: )Future\.wait\(\[_referralsFuture, _viralCoefficientFuture\]\),',
    r'\1_combinedFuture,',
    content
)

with open("srcs/app/lib/screens/referrals_dashboard_screen.dart", "w") as f:
    f.write(content)
