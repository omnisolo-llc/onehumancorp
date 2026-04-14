with open("srcs/app/lib/widgets/growth_referral_widget.dart", "r") as f:
    code = f.read()

new_code = code.replace(
    '''await ref.read(apiServiceProvider)!.createReferral(
                              "anonymous",
                              "xYz8vQ_local_sovereign",
                            );''',
    '''await ref.read(apiServiceProvider)!.createReferral(
                              "anonymous",
                              "xYz8vQ_local_sovereign",
                            );
                            await ref.read(apiServiceProvider)!.trackSovereignToCloudInvite(
                              "anonymous",
                              "asset_market_audit",
                            );'''
)

with open("srcs/app/lib/widgets/growth_referral_widget.dart", "w") as f:
    f.write(new_code)
