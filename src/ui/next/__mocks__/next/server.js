class NextResponse extends Response {
  constructor(body, init) {
    super(body, init);
  }
  static json(body, init) {
    return new Response(JSON.stringify(body), {
      status: init?.status || 200,
      headers: { 'Content-Type': 'application/json', ...(init?.headers || {}) },
    });
  }
}
export { NextResponse };
