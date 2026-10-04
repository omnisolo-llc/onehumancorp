export interface ButtonStateObservation {
  states: Array<{ disabled: boolean; text: string | null }>;
  disconnect(): void;
}

export function observeButtonStates(button: HTMLButtonElement): ButtonStateObservation;
