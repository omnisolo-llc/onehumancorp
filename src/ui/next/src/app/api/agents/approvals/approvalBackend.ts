const APPROVAL_ID = /^[A-Za-z0-9._-]{1,128}$/;

export function approvalBackendPath(id: unknown): string {
  if (typeof id !== "string" || !APPROVAL_ID.test(id)) {
    throw new Error("invalid approval ID");
  }
  return `/api/agents/approvals/${id}`;
}

export function privateApprovalError(status: number, message: string): Response {
  return Response.json(
    { error: message },
    {
      status,
      headers: {
        "cache-control": "private, no-store",
        pragma: "no-cache",
        "x-content-type-options": "nosniff",
      },
    },
  );
}
