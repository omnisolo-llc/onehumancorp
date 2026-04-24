import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/router.dart';

void main() {
  group('computeInitialLocation', () {
    test('returns /landing for web', () {
      expect(computeInitialLocation(isWeb: true, isStandaloneOverride: false), '/landing');
      expect(computeInitialLocation(isWeb: true, isStandaloneOverride: true), '/landing');
    });

    test('returns /dashboard for standalone mode (non-web)', () {
      expect(computeInitialLocation(isWeb: false, isStandaloneOverride: true), '/dashboard');
    });

    test('returns /login for regular app (non-web, non-standalone)', () {
      expect(computeInitialLocation(isWeb: false, isStandaloneOverride: false), '/login');
    });
  });

  group('computeRedirect', () {
    group('Web mode', () {
      test('redirects non-landing routes to /landing', () {
        expect(
          computeRedirect(
            matchedLocation: '/dashboard',
            isLoggedIn: false,
            isWeb: true,
            isStandaloneOverride: false,
            redirectTarget: null,
            fullPath: '/dashboard',
          ),
          '/landing',
        );
      });

      test('allows /landing route', () {
        expect(
          computeRedirect(
            matchedLocation: '/landing',
            isLoggedIn: false,
            isWeb: true,
            isStandaloneOverride: false,
            redirectTarget: null,
            fullPath: '/landing',
          ),
          isNull,
        );
      });
    });

    group('Standalone mode', () {
      test('redirects /landing and /login to /dashboard', () {
        expect(
          computeRedirect(
            matchedLocation: '/landing',
            isLoggedIn: false,
            isWeb: false,
            isStandaloneOverride: true,
            redirectTarget: null,
            fullPath: '/landing',
          ),
          '/dashboard',
        );
        expect(
          computeRedirect(
            matchedLocation: '/login',
            isLoggedIn: false,
            isWeb: false,
            isStandaloneOverride: true,
            redirectTarget: null,
            fullPath: '/login',
          ),
          '/dashboard',
        );
      });

      test('allows other routes', () {
        expect(
          computeRedirect(
            matchedLocation: '/dashboard',
            isLoggedIn: false,
            isWeb: false,
            isStandaloneOverride: true,
            redirectTarget: null,
            fullPath: '/dashboard',
          ),
          isNull,
        );
      });
    });

    group('Regular mode', () {
      test('redirects to /login if not logged in and not on login route', () {
        expect(
          computeRedirect(
            matchedLocation: '/dashboard',
            isLoggedIn: false,
            isWeb: false,
            isStandaloneOverride: false,
            redirectTarget: null,
            fullPath: '/dashboard',
          ),
          '/login?redirect=%2Fdashboard',
        );
      });

      test('allows /login route when not logged in', () {
        expect(
          computeRedirect(
            matchedLocation: '/login',
            isLoggedIn: false,
            isWeb: false,
            isStandaloneOverride: false,
            redirectTarget: null,
            fullPath: '/login',
          ),
          isNull,
        );
      });

      test('redirects to dashboard if logged in and on login route', () {
        expect(
          computeRedirect(
            matchedLocation: '/login',
            isLoggedIn: true,
            isWeb: false,
            isStandaloneOverride: false,
            redirectTarget: null,
            fullPath: '/login',
          ),
          '/dashboard',
        );
      });

      test('redirects to redirect target if logged in and on login route', () {
        expect(
          computeRedirect(
            matchedLocation: '/login',
            isLoggedIn: true,
            isWeb: false,
            isStandaloneOverride: false,
            redirectTarget: '/settings',
            fullPath: '/login',
          ),
          '/settings',
        );
      });

      test('redirects to dashboard if logged in and on landing route', () {
        expect(
          computeRedirect(
            matchedLocation: '/landing',
            isLoggedIn: true,
            isWeb: false,
            isStandaloneOverride: false,
            redirectTarget: null,
            fullPath: '/landing',
          ),
          '/dashboard',
        );
      });
    });
  });
}
