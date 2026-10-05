// These obsolete public-directory implementations have backend-backed Next pages.
// This map is intentionally exact and is applied only after authentication.
const destinations = new Map<string, string>([
  ["/integrations.html", "/integrations"],
  ["/ui/integrations.html", "/integrations"],
  ["/api-docs.html", "/api-docs"],
  ["/ui/api-docs.html", "/api-docs"],
  ["/api/ui/api-docs.html", "/api-docs"],
  ["/api/v1/ui/api-docs.html", "/api-docs"],
  ["/trial-extension.html", "/trial-extension"],
  ["/ui/trial-extension.html", "/trial-extension"],
  ["/changelog.html", "/changelog"],
  ["/ui/changelog.html", "/changelog"],
  ["/api/ui/changelog.html", "/changelog"],
  ["/api/v1/ui/changelog.html", "/changelog"],
  ["/unified-feed.html", "/unified-feed"],
  ["/ui/unified-feed.html", "/unified-feed"],
  ["/booking-create.html", "/services/new"],
  ["/ui/booking-create.html", "/services/new"],
]);

export function retiredPageDestination(pathname: string): string | null {
  return destinations.get(pathname) ?? null;
}
