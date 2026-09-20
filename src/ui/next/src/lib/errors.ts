/** Extract a displayable message without assuming every thrown value is an Error.
 * Callers remain responsible for using non-sensitive errors at provider boundaries.
 */
export function errorMessage(
  error: unknown,
  fallback = 'The request could not be completed.',
): string {
  if (typeof error === 'string') return error.trim() ? error : fallback;
  try {
    if (error !== null && typeof error === 'object' && 'message' in error) {
      const message: unknown = error.message;
      if (typeof message === 'string' && message.trim()) return message;
    }
  } catch {
    // Proxies/getters can throw; reporting an error must not fail a second time.
  }
  return fallback;
}
