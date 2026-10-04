// These obsolete public-directory implementations have backend-backed Next pages.
// This map is intentionally exact and is applied only after authentication.
const destinations = new Map<string, string>([
  ["/integrations.html", "/integrations"],
  ["/ui/integrations.html", "/integrations"],
  ["/api-docs.html", "/api-docs"],
  ["/ui/api-docs.html", "/api-docs"],
  ["/api/ui/api-docs.html", "/api-docs"],
  ["/api/v1/ui/api-docs.html", "/api-docs"],
]);

export function retiredPageDestination(pathname: string): string | null {
  return destinations.get(pathname) ?? null;
}
