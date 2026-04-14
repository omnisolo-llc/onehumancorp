with open("srcs/app/test/growth_referral_widget_test.dart", "r") as f:
    code = f.read()

new_code = code.replace(
    'when(() => mockApiService.createReferral(any(), any())).thenAnswer((_) async {});',
    '''when(() => mockApiService.createReferral(any(), any())).thenAnswer((_) async {});
    when(() => mockApiService.trackSovereignToCloudInvite(any(), any())).thenAnswer((_) async {});'''
)

new_code = new_code.replace(
    'verify(() => mockApiService.createReferral("anonymous", "xYz8vQ_local_sovereign")).called(1);',
    '''verify(() => mockApiService.createReferral("anonymous", "xYz8vQ_local_sovereign")).called(1);
    verify(() => mockApiService.trackSovereignToCloudInvite("anonymous", "asset_market_audit")).called(1);'''
)

with open("srcs/app/test/growth_referral_widget_test.dart", "w") as f:
    f.write(new_code)
