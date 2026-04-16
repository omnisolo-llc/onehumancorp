import re

with open("srcs/server/dashboard/handlers_growth.go", "r") as f:
    content = f.read()

# Let's see what methods we accidentally matched in plan_review_fix.py
# The regex `break\n\t\t\}\n\t\}` matched `handleTeamInviteAccept` but also `handleReferralClick` and `handleReferralConvert`
# Because it was not specific enough.
# Let's fix handleReferralClick (line 398ish) and handleReferralConvert (line 438ish)

# First, remove all occurrences of `if found && s.viralLoopTracker != nil { s.viralLoopTracker.RecordInviteAccepted(r.Context(), updated.InviteeID) }`
content = re.sub(
    r'\n\n\t*if found && s\.viralLoopTracker != nil \{\n\t\t*s\.viralLoopTracker\.RecordInviteAccepted\(r\.Context\(\), updated\.InviteeID\)\n\t*\}',
    r'',
    content
)

# And only add it inside handleTeamInviteAccept
content = re.sub(
    r'(func \(s \*Server\) handleTeamInviteAccept\(w http\.ResponseWriter, r \*http\.Request\) \{.*?\n\n)(\t*if !found \{)',
    r'\1\tif found && s.viralLoopTracker != nil {\n\t\ts.viralLoopTracker.RecordInviteAccepted(r.Context(), updated.InviteeID)\n\t}\n\n\2',
    content,
    flags=re.DOTALL
)

with open("srcs/server/dashboard/handlers_growth.go", "w") as f:
    f.write(content)
