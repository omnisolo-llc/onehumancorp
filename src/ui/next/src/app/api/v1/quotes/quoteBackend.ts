import { validateJsonRequestBody } from "@/lib/auth/backendTransport";

const QUOTE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const EMPTY_QUOTE_BODY = new TextEncoder().encode("{}");

export function validateLegacyQuoteBody(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  try {
    return validateJsonRequestBody(body);
  } catch {
    return EMPTY_QUOTE_BODY;
  }
}

export function quoteBackendPath(id: unknown, suffix = ""): string {
  if (
    typeof id !== "string" ||
    id === "." ||
    id === ".." ||
    !QUOTE_ID.test(id)
  ) {
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

export function validateQuoteAcceptanceBody(body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> {
  const text = new TextDecoder("utf-8", { fatal: true }).decode(body);
  const value: unknown = text.trim() ? JSON.parse(text) : {};
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("expected a quote acceptance object");
  }
  const fields = value as Record<string, unknown>;
  if (Object.keys(fields).some(key => key !== "expected_updated_at")) {
    throw new Error("unexpected quote acceptance field");
  }
  const timestamp = fields.expected_updated_at;
  if (timestamp === undefined) return new TextEncoder().encode("{}");
  if (typeof timestamp !== "string" || timestamp.length > 64
      || !/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,9})?(?:Z|[+-]\d{2}:\d{2})$/.test(timestamp)
      || !Number.isFinite(Date.parse(timestamp))) {
    throw new Error("invalid observed quote version");
  }
  return new TextEncoder().encode(JSON.stringify({ expected_updated_at: timestamp }));
}
