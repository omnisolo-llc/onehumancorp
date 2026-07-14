const SUBSCRIPTION_ID = /^[A-Za-z0-9._-]{1,128}$/;

export function subscriptionBackendPath(id: unknown, suffix = ""): string {
  if (typeof id !== "string" || !SUBSCRIPTION_ID.test(id)) {
    throw new Error("invalid subscription ID");
  }
  return `/api/subscriptions/${id}${suffix}`;
}

export function invalidSubscriptionId(): Response {
  return Response.json(
    { error: "invalid subscription ID" },
    {
      status: 400,
      headers: {
        "cache-control": "private, no-store",
        pragma: "no-cache",
        "x-content-type-options": "nosniff",
      },
    },
  );
}
