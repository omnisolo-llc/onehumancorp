export class NextResponse extends Response {
  constructor(body?: BodyInit | null, init?: ResponseInit) {
    super(body, init);
  }
  static json(body: any, init?: ResponseInit) {
    return new Response(JSON.stringify(body), {
      status: init?.status || 200,
      headers: { 'Content-Type': 'application/json', ...(init?.headers || {}) },
    });
  }
}
