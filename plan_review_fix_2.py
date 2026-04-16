import re

with open("srcs/server/dashboard/handlers_growth.go", "r") as f:
    content = f.read()

# Let's clean up referrals conversion dead code
with open("srcs/server/services/growth/referrals.go", "r") as f:
    ref_content = f.read()

ref_content = re.sub(
    r'(var inviteConversionsCounter metric\.Int64Counter)',
    r'',
    ref_content
)

ref_content = re.sub(
    r'(inviteConversionsCounter, _ = meter\.Int64Counter\("growth_invite_conversions_total"\))',
    r'',
    ref_content
)

ref_content = re.sub(
    r'(func \(rt \*ReferralTracker\) RecordInviteConversion\(ctx context\.Context\) \{\n\tif inviteConversionsCounter != nil \{\n\t\tinviteConversionsCounter\.Add\(ctx, 1\)\n\t\}\n\})',
    r'',
    ref_content
)

with open("srcs/server/services/growth/referrals.go", "w") as f:
    f.write(ref_content)
