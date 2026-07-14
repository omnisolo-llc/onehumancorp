const ARTICLE_ID = /^[A-Za-z0-9._-]{1,128}$/;

export function helpArticleBackendPath(id: unknown): string {
  if (typeof id !== "string" || !ARTICLE_ID.test(id)) {
    throw new Error("invalid article ID");
  }
  return `/api/help/${id}`;
}

export function invalidArticleId(): Response {
  return Response.json(
    { error: "invalid article ID" },
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
