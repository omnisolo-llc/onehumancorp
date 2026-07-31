
import React, { useState } from 'react';
import { Box, Text } from 'ink';
import TextInput from 'ink-text-input';

interface ChatResponse {
  intent: 'Support' | 'Sales' | 'Billing' | 'Handoff' | 'General';
  handoff_required: boolean;
  auto_reply: string | null;
  copilot_draft: string | null;
}

export const OmnichannelChat: React.FC<{ onBack?: () => void }> = ({ onBack }) => {
  const [message, setMessage] = useState('');
  const [isCopilotMode, setIsCopilotMode] = useState(false);
  const [response, setResponse] = useState<ChatResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (val: string) => {
    if (val.trim() === 'back' && onBack) {
      onBack();
      return;
    }
    if (val.trim() === 'toggle copilot') {
      setIsCopilotMode(!isCopilotMode);
      setMessage('');
      return;
    }

    setIsSubmitting(true);
    setError(null);
    setResponse(null);
    setMessage('');

    try {
      const res = await fetch('http://127.0.0.1:4000/rpc', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          jsonrpc: '2.0',
          method: 'omnichannel_chat_process',
          params: {
            message: val,
            is_copilot_mode: isCopilotMode,
          },
          id: 1,
        }),
      });

      const data = await res.json();
      if (data.error) {
        setError(data.error.message);
      } else {
        setResponse(data.result as ChatResponse);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="magenta" padding={1}>
      <Text bold color="magenta">Omnichannel Chat Testing</Text>
      <Text color="gray">Type "back" to return to menu. Type "toggle copilot" to toggle mode.</Text>

      <Box marginTop={1}>
        <Text color="yellow">Copilot Mode: </Text>
        <Text bold>{isCopilotMode ? 'ON' : 'OFF'}</Text>
      </Box>

      {error && (
        <Box marginTop={1} padding={1} borderStyle="single" borderColor="red">
          <Text color="red">{error}</Text>
        </Box>
      )}

      {response && (
        <Box marginTop={1} flexDirection="column" padding={1} borderStyle="single" borderColor="green">
          <Text color="green">Response Intent: {response.intent}</Text>
          <Text color="green">Handoff Required: {response.handoff_required ? 'Yes' : 'No'}</Text>
          {response.auto_reply && <Text color="blue">Auto Reply: {response.auto_reply}</Text>}
          {response.copilot_draft && <Text color="cyan">Copilot Draft: {response.copilot_draft}</Text>}
        </Box>
      )}

      <Box marginTop={1}>
        <Text color="blue">{isSubmitting ? 'Processing... ' : 'Message > '}</Text>
        {!isSubmitting && (
          <TextInput
            value={message}
            onChange={setMessage}
            onSubmit={handleSubmit}
          />
        )}
      </Box>
    </Box>
  );
};
