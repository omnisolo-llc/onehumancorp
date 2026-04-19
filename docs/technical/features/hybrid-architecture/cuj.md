<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Architecture CUJ

1. **Standalone Offline Work:** Operator takes their device offline and continues interacting with the Standalone Desktop Mode, utilizing local SQLite.
2. **Reconnection & Sync:** Operator reconnects to the network. The `AutoDreamWorker` sync engine activates and syncs local memories to the Cloud PostgreSQL instance.
3. **Cloud Processing:** Cloud-native agents process the synced memories.
4. **Seamless Degradation:** If cloud services become unavailable, the system transparently falls back to Standalone Desktop Mode.

</div>
