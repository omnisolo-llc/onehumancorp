import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const appShellSource = readFileSync(join(__dirname, 'AppShell.tsx'), 'utf8');
const rootLayoutSource = readFileSync(join(__dirname, '..', 'layout.tsx'), 'utf8');

describe('VoiceAssistant ownership', () => {
  it('is rendered once by AppShell instead of RootLayout', () => {
    expect(appShellSource.match(/<VoiceAssistant\s*\/>/g)).toHaveLength(1);
    expect(rootLayoutSource).not.toMatch(/<VoiceAssistant\s*\/>/);
  });
});
