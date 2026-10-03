/** Preserve upstream failures and accept only an actual completed text result. */
export async function runtimeTextResponse(response: Response): Promise<Response> {
  if (!response.ok) return response;
  const unconfirmed = () => Response.json({ error: 'Backend returned no confirmed runtime output' }, { status: 502 });
  if (response.status !== 200) return unconfirmed();
  try {
    const payload = await response.json();
    if (payload?.error != null) {
      const message = typeof payload.error === 'string' ? payload.error : payload.error.message;
      return Response.json({ error: typeof message === 'string' && message.trim() ? message : 'Runtime execution failed' }, { status: 502 });
    }
    const result = payload?.result;
    if (!result || typeof result !== 'object' || Array.isArray(result)
      || result.error != null || result.success !== undefined && result.success !== true
      || result.status !== undefined && result.status !== 'completed' && result.status !== 'success'
      || typeof result.output !== 'string' || !result.output.trim()) return unconfirmed();
    return Response.json({ result: result.output });
  } catch {
    return Response.json({ error: 'Backend returned an invalid response' }, { status: 502 });
  }
}
