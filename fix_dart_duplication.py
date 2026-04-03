import re

with open('./srcs/app/lib/screens/user_management_screen.dart', 'r') as f:
    content = f.read()

# It seems `_GrowthReferralWidgetState` is declared twice in `user_management_screen.dart`. Let's remove the second one.
parts = content.split("class GrowthReferralWidget extends StatefulWidget {")
if len(parts) > 2:
    # Meaning there are two declarations. Let's keep the first and the rest of the file until the end of the first declaration, then remove the second one.
    pass

# A simpler way: we patched it multiple times! Let's check how many times `class GrowthReferralWidget extends StatefulWidget` appears.
