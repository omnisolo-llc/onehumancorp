export class NextResponse extends Response {
  constructor(body?: BodyInit | null, init?: ResponseInit) {
    super(body, init);
  }
  static json(body: any, init?: ResponseInit) {
    return new Response(JSON.stringify(body), {
      ...init,
      headers: { ...init?.headers, 'Content-Type': 'application/json' },
    });
  }
}
export class NextRequest extends Request {}
