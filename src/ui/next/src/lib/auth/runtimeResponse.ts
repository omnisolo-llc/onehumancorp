/** Preserve upstream failures and accept only an actual completed text result. */
export async function runtimeTextResponse(response: Response): Promise<Response> {
  if (!response.ok) return response;
  const json = (data: unknown, status = 200) => Response.json(data, { status, headers: { 'Cache-Control': 'private, no-store' } });
  const unconfirmed = () => json({ error: 'Backend returned no confirmed runtime output' }, 502);
  if (response.status !== 200) return unconfirmed();
  try {
    const payload = await response.json();
    if (payload?.error != null) {
      const message = typeof payload.error === 'string' ? payload.error : payload.error.message;
      return json({ error: typeof message === 'string' && message.trim() ? message : 'Runtime execution failed' }, 502);
    }
    if (payload?.success !== undefined && payload.success !== true
      || payload?.status !== undefined && payload.status !== 'completed' && payload.status !== 'success') return unconfirmed();
    const result = payload?.result;
    if (!result || typeof result !== 'object' || Array.isArray(result)
      || result.error != null || result.success !== undefined && result.success !== true
      || result.status !== undefined && result.status !== 'completed' && result.status !== 'success'
      || typeof result.output !== 'string' || !result.output.trim()) return unconfirmed();
    return json({ result: result.output });
  } catch {
    return json({ error: 'Backend returned an invalid response' }, 502);
  }
}
