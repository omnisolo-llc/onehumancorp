const QUOTE_ID = /^[A-Za-z0-9._-]{1,128}$/;

export function quoteBackendPath(id: unknown, suffix = ""): string {
  if (typeof id !== "string" || !QUOTE_ID.test(id)) {
    throw new Error("invalid quote ID");
  }
  return `/api/v1/quotes/${id}${suffix}`;
}

export function quoteIdFromUrl(url: string): string | null {
  const id = new URL(url).searchParams.get("id");
  if (id === null) return null;
  quoteBackendPath(id);
  return id;
}

export function invalidQuoteId(): Response {
  return Response.json(
    { error: "invalid quote ID" },
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
