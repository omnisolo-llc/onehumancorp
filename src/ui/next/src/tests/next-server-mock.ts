export const NextResponse = {
  json: (data: any, init?: any) => {
    return new Response(JSON.stringify(data), {
      status: init?.status || 200,
      headers: {
        'Content-Type': 'application/json',
        ...init?.headers
      }
    });
  }
};