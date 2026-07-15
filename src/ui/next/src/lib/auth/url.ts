const RAW_CONTROL_OR_SEPARATOR = /[\\\u0000-\u001f\u007f]|\/\//;
const ENCODED_AMBIGUITY = /%(?:0[0-9a-f]|1[0-9a-f]|2e|2f|5c|7f|25)/i;
const MALFORMED_PERCENT = /%(?![0-9a-f]{2})/i;
const DOT_SEGMENT = /(?:^|\/)\.{1,2}(?:\/|$)/;

export function canonicalRawPath(pathname: string): string {
  if (
    !pathname.startsWith("/") ||
    RAW_CONTROL_OR_SEPARATOR.test(pathname) ||
    ENCODED_AMBIGUITY.test(pathname) ||
    MALFORMED_PERCENT.test(pathname) ||
    DOT_SEGMENT.test(pathname)
  ) {
    throw new Error("ambiguous request path");
  }
  return pathname;
}

export function safeReturnPath(value: string | null | undefined): string {
  if (!value || !value.startsWith("/") || value.startsWith("//")) return "/dashboard";
  if (
    RAW_CONTROL_OR_SEPARATOR.test(value) ||
    ENCODED_AMBIGUITY.test(value) ||
    MALFORMED_PERCENT.test(value)
  ) {
    return "/dashboard";
  }
  const path = value.split(/[?#]/, 1)[0];
  try {
    canonicalRawPath(path);
  } catch {
    return "/dashboard";
  }
  if (path === "/login" || path === "/api/auth/login") return "/dashboard";
  return value;
}
