import LocalSubprocessRuntime from './lib/index.js';
import {localServiceSpawnSpec} from './local-services-env.mjs';

// Retain the pinned implementation's process ownership, credential scrub,
// sandbox policy and teardown; supply only the issued local service lease.
export default class OmniSoloLocalSubprocess extends LocalSubprocessRuntime {
  spawn(spec) {
    return super.spawn(localServiceSpawnSpec(spec));
  }
  spawnTerminal(spec) {
    return super.spawnTerminal(localServiceSpawnSpec(spec));
  }
}
