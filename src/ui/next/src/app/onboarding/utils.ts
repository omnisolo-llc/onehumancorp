export function generateSubdomain(name: string): string {
  if (!name || name.trim() === '') return 'my-business.ohc.app';
  const cleanName = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return cleanName ? `${cleanName}.ohc.app` : 'my-business.ohc.app';
}
