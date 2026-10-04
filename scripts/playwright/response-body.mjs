/** Start observing actual bytes as soon as headers arrive, before later UI actions. */
export async function captureResponseBody(pending) {
  const response = await pending;
  const body = await response.body();
  return { response, body: async () => Buffer.from(body) };
}
