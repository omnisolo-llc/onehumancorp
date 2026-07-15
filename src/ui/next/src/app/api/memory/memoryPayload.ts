const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();
const SAFE_MEMORY_ID = /^[A-Za-z0-9._-]{1,200}$/;

function parseObject(body: Uint8Array<ArrayBuffer>): Record<string, unknown> {
  const parsed = JSON.parse(decoder.decode(body)) as unknown;
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error("request must be an object");
  }
  return parsed as Record<string, unknown>;
}

function boundedString(value: unknown, maximum: number, field: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`${field} is required`);
  }
  let count = 0;
  for (const _character of value) {
    count += 1;
    if (count > maximum) throw new Error(`${field} is too long`);
  }
  return value;
}

export function memoryId(value: string): string {
  if (!SAFE_MEMORY_ID.test(value)) throw new Error("invalid memory id");
  return value;
}

export function forgetMemoryRequest(id: string) {
  const safeId = memoryId(id);
  return (_body: Uint8Array<ArrayBuffer>): Uint8Array<ArrayBuffer> =>
    encoder.encode(JSON.stringify({ action: "forget", id: safeId }));
}

export function importMemoryRequest(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  const input = parseObject(body);
  const content = boundedString(input.content, 750_000, "content");
  const source = boundedString(input.source_type, 255, "source_type");
  return encoder.encode(
    JSON.stringify({ action: "import", content, scope: "global", source }),
  );
}

export function crossSessionMemoryRequest(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  const input = parseObject(body);
  const query = boundedString(input.query, 500, "query");
  const limit = input.limit === undefined ? 5 : input.limit;
  const summarize = input.summarize === undefined ? false : input.summarize;
  if (!Number.isInteger(limit) || (limit as number) < 1 || (limit as number) > 20) {
    throw new Error("invalid limit");
  }
  if (typeof summarize !== "boolean") throw new Error("invalid summarize value");
  return encoder.encode(JSON.stringify({ query, limit, summarize }));
}

export function memoryCustomerPath(customerId: string): string {
  return `/api/memory/summary/${memoryId(customerId)}`;
}

export function invalidMemoryId(): Response {
  return Response.json({ error: "invalid memory id" }, { status: 400 });
}
