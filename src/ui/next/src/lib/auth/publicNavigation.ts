/** Public auth pages accept document navigation; anonymous Flight stays protected. */
export function navigateToPublicAuth(
  path: "/login" | "/register" | "/verify-email",
  replace = false,
): void {
  if (replace) window.location.replace(path);
  else window.location.assign(path);
}
