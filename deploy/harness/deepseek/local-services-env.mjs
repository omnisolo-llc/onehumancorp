// Explicit service capabilities for native tool children. The SDK still
// scrubs all other ambient credentials before merging this spawn environment.
export function localServiceSpawnSpec(spec, environment = process.env) {
  const url = environment.OMNISOLO_LOCAL_SERVICE_URL;
  const token = environment.OMNISOLO_LOCAL_SERVICE_TOKEN;
  if (!url && !token) return spec;
  if (!url || !token) throw new Error('incomplete local service capability');
  const parsed = new URL(url);
  if (parsed.protocol !== 'http:' || parsed.hostname !== '127.0.0.1'
      || parsed.username || parsed.password || parsed.search || parsed.hash
      || !parsed.port || parsed.pathname !== '/v1' || /\s/.test(token)) {
    throw new Error('invalid local service capability');
  }
  return {...spec, env: {...spec.env,
    OMNISOLO_LOCAL_SERVICE_URL: url, OMNISOLO_LOCAL_SERVICE_TOKEN: token}};
}
