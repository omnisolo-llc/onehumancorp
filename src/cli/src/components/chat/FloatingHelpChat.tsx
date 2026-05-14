import React, { useState } from 'react';
import { Box, Text } from 'ink';
import { PromptInput } from '../PromptInput';

export const FloatingHelpChat = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<{role: 'user' | 'agent', text: string}[]>([]);

  if (!isOpen) {
    return (
      <Box padding={1} borderStyle="round" borderColor="blue">
        <Text color="blue">Press 'h' to chat with the Help Agent</Text>
      </Box>
    );
  }

  return (
    <Box flexDirection="column" borderStyle="double" borderColor="blue" padding={1} width={60}>
      <Text color="blue" bold>AI Support Chat</Text>
      <Box flexDirection="column" marginTop={1} marginBottom={1} minHeight={5}>
        {messages.length === 0 ? (
          <Text color="gray">Hi! I'm your AI Support Agent. How can I help you run your business today?</Text>
        ) : (
          messages.map((m, i) => (
            <Box key={i}>
              <Text color={m.role === 'user' ? 'green' : 'blue'}>{m.role === 'user' ? 'You: ' : 'Agent: '}</Text>
              <Text>{m.text}</Text>
            </Box>
          ))
        )}
      </Box>
      <PromptInput
        onSubmit={(val) => {
          setMessages([...messages, { role: 'user', text: val }, { role: 'agent', text: "I can help with that! Please check the Help Center for a detailed guide." }]);
        }}
        promptText="Ask..."
      />
      <Text color="gray">Press 'Esc' to close chat</Text>
    </Box>
  );
};
