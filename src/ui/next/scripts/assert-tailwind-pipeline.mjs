import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'

const packageJson = JSON.parse(
  await readFile(new URL('../package.json', import.meta.url), 'utf8'),
)
const postcssConfig = await readFile(
  new URL('../postcss.config.mjs', import.meta.url),
  'utf8',
)

assert.match(packageJson.devDependencies.tailwindcss, /^\^?3\./)
assert.equal(
  packageJson.devDependencies['@tailwindcss/postcss'],
  undefined,
  'Tailwind 4 PostCSS must not be installed',
)
assert.equal(
  packageJson.postcss,
  undefined,
  'package.json must not compete with postcss.config.mjs',
)
assert.match(postcssConfig, /tailwindcss:\s*\{\}/)

process.stdout.write('Tailwind/PostCSS pipeline is coherent.\n')
