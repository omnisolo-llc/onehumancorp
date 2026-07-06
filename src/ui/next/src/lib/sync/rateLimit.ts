export function checkRateLimit(res: Response) {
  if (res.status === 429) {
    console.warn("Rate limit exceeded during sync.");
  }
}
