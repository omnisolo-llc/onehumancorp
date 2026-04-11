const { execSync } = require('child_process');
const path = require('path');

// Reconstruct arguments
const args = process.argv.slice(2);
let protocPath = args[0];
const remainingArgs = args.slice(1);

// Execute protoc with the provided args, no custom plugin path injection needed as it is failing
try {
  // It fails with "not found" because the child_process uses sh, let's just run it natively.
  execSync(`${protocPath} ${remainingArgs.join(' ')}`, { stdio: 'inherit' });
} catch (error) {
  process.exit(error.status || 1);
}
