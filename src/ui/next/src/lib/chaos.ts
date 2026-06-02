export interface FaultConfig {
  latencyMs?: number;      // Fixed delay before responding
  jitterMs?: number;       // Random additional delay up to this value
  timeout?: boolean;       // Simulate a hard timeout (throws after delay)
  errorRate?: number;      // Probability (0.0 to 1.0) of throwing an error
  customError?: Error;     // Specific error to throw
}

export class FaultInjector {
  private static configs = new Map<string, FaultConfig>();

  static setConfig(point: string, config: FaultConfig): void {
    this.configs.set(point, config);
  }

  static clearConfig(point: string): void {
    this.configs.delete(point);
  }

  static clearAll(): void {
    this.configs.clear();
  }

  static async applyFault(point: string): Promise<void> {
    if (process.env.NODE_ENV === 'production' && process.env.ENABLE_CHAOS !== 'true') return;
    const config = this.configs.get(point);
    if (!config) return;

    if (config.latencyMs) {
      const delay = config.latencyMs + (config.jitterMs ? Math.random() * config.jitterMs : 0);
      await new Promise(resolve => setTimeout(resolve, delay));
    }

    if (config.timeout) {
      throw new Error(`Timeout Fault Injected at ${point}`);
    }

    if (config.errorRate && Math.random() < config.errorRate) {
      throw config.customError || new Error(`Random Fault Injected at ${point}`);
    }
  }
}
