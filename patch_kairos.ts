import fs from 'fs';

const p = 'src/ui/next/src/app/kairos/page.tsx';
let data = fs.readFileSync(p, 'utf8');

data = data.replace(/Size": "842\.5 MB" \}\);\n        \}\n        \}/g, "Size\": \"842.5 MB\" });\n        }");
fs.writeFileSync(p, data);
