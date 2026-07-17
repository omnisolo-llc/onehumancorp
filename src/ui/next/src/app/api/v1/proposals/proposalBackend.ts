import { validateJsonRequestBody } from "@/lib/auth/backendTransport";

const PROPOSAL_ID = /^[A-Za-z0-9._-]{1,128}$/;
const EMPTY_PROPOSAL_BODY = new TextEncoder().encode("{}");

export function validateLegacyProposalBody(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  try {
    return validateJsonRequestBody(body);
  } catch {
    return EMPTY_PROPOSAL_BODY;
  }
}

export function proposalBackendPath(id: unknown, suffix = ""): string {
  if (
    typeof id !== "string" ||
    id === "." ||
    id === ".." ||
    !PROPOSAL_ID.test(id)
  ) {
    throw new Error("invalid proposal ID");
  }
  return `/api/v1/proposals/${id}${suffix}`;
}

export function proposalIdFromUrl(url: string): string | null {
  const id = new URL(url).searchParams.get("id");
  if (id === null) return null;
  proposalBackendPath(id);
  return id;
}

export function invalidProposalId(): Response {
  return Response.json(
    { error: "invalid proposal ID" },
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
