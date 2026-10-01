export type ContactCard = {
  name: string; title?: string; company?: string; phone?: string;
  email?: string; website?: string; linkedin?: string;
};

function escapeValue(value: string) {
  return value.replace(/\\/g, '\\\\').replace(/\r\n|\r|\n/g, '\\n').replace(/;/g, '\\;').replace(/,/g, '\\,');
}

export function downloadContactCard(card: ContactCard) {
  if (!card.name.trim()) throw new Error('A contact name is required');
  const lines = ['BEGIN:VCARD', 'VERSION:3.0', `N:;${escapeValue(card.name)};;;`, `FN:${escapeValue(card.name)}`];
  for (const [field, value] of [
    ['TITLE', card.title], ['ORG', card.company], ['TEL;TYPE=WORK,VOICE', card.phone],
    ['EMAIL;TYPE=PREF,INTERNET', card.email], ['URL', card.website], ['URL;TYPE=LinkedIn', card.linkedin],
  ]) if (value) lines.push(`${field}:${escapeValue(value)}`);
  lines.push('END:VCARD');
  const url = URL.createObjectURL(new Blob([lines.join('\r\n') + '\r\n'], { type: 'text/vcard;charset=utf-8' }));
  const link = document.createElement('a');
  link.href = url;
  link.download = `${card.name.replace(/[^\p{L}\p{N}._-]+/gu, '_') || 'Contact'}_Contact.vcf`;
  document.body.appendChild(link);
  try { link.click(); }
  finally { link.remove(); setTimeout(() => URL.revokeObjectURL(url), 0); }
}
