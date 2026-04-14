with open("srcs/app/lib/screens/landing_screen.dart", "r") as f:
    code = f.read()
    if 'trackSovereignToCloudInvite' in code:
        print("trackSovereignToCloudInvite in landing screen")
    else:
        print("not in landing screen")
