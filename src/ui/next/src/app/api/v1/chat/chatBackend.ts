const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();
const MAX_MESSAGE_LENGTH = 1_000;

export function normalizeChatBody(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  const value = JSON.parse(decoder.decode(body));
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("invalid chat request");
  }
  const message = (value as Record<string, unknown>).message;
  if (typeof message !== "string") throw new Error("invalid chat request");
  const trimmed = message.trim();
  if (trimmed.length === 0 || trimmed.length > MAX_MESSAGE_LENGTH) {
    throw new Error("invalid chat request");
  }
  return encoder.encode(JSON.stringify({ message: trimmed }));
}
