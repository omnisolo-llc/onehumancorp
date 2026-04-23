import React, { useState } from 'react';
import { Box, Text } from 'ink';
import { AgentStatus } from './components/AgentStatus';
import { ToolProgress } from './components/ToolProgress';
import { MarkdownText } from './components/MarkdownText';
import { InteractivePrompt } from './components/InteractivePrompt';
import { useOrchestrator } from './hooks/useOrchestrator';

export const App = () => {
  const { status, tools } = useOrchestrator();
  const [messages, setMessages] = useState<string[]>([]);
  const markdown = `# OHC Interactive Harness\n\n- Powered by Ink\n- React in the CLI`;

  const handleCommand = (cmd: string) => {
    if (cmd.trim()) {
      setMessages([...messages, `> ${cmd}`]);
    }
  };

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1} width={80}>
      <Box justifyContent="center" marginBottom={1}>
        <Text bold color="white" backgroundColor="blue"> ONE HUMAN CORP </Text>
        <Text> - Standalone Agent Mode </Text>
      </Box>

      <AgentStatus status={status} />
      <ToolProgress tools={tools} />

      <Box borderStyle="single" borderColor="gray" padding={1} marginTop={1} flexDirection="column">
        <MarkdownText content={markdown} />
        {messages.map((msg, idx) => (
          <Box key={idx} marginTop={1}>
            <Text color="yellow">{msg}</Text>
          </Box>
        ))}
      </Box>
      <Box marginTop={1}>
        <InteractivePrompt onSubmit={handleCommand} />
      </Box>
    </Box>
  );
};
