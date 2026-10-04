export type CapturedResponse<Response> = { response: Response; body(): Promise<Buffer> };
export function captureResponseBody<Response extends { body(): Promise<Buffer> }>(
  pending: Promise<Response>,
): Promise<CapturedResponse<Response>>;
