import re

with open("srcs/app/lib/router.dart", "r") as f:
    content = f.read()

content = content.replace("initialLocation: '/kairos',", "initialLocation: '/landing',")
content = content.replace(
    "redirect: (context, state) => null,",
    """redirect: (context, state) {
      final isLoggedIn = authState.valueOrNull != null;
      final isLoginRoute = state.matchedLocation == '/login';
      final isLandingRoute = state.matchedLocation == '/landing';

      if (!isLoggedIn && !isLoginRoute && !isLandingRoute) return '/landing';
      if (isLoggedIn && isLoginRoute) return '/dashboard';
      return null;
    },"""
)

with open("srcs/app/lib/router.dart", "w") as f:
    f.write(content)
