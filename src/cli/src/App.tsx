import React from 'react';
import { Box, Text } from 'ink';
import { AgentStatus } from './components/AgentStatus';
import { ToolProgress } from './components/ToolProgress';
import { MarkdownText } from './components/MarkdownText';
import { useOrchestrator } from './hooks/useOrchestrator';

export const App = () => {
  const { status, tools } = useOrchestrator();
  const markdown = `# OHC Interactive Harness\n\n- Powered by Ink\n- React in the CLI`;

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1} width={80}>
      <Box justifyContent="center" marginBottom={1}>
        <Text bold color="white" backgroundColor="blue"> ONE HUMAN CORP </Text>
        <Text> - Standalone Agent Mode </Text>
      </Box>

      <AgentStatus status={status} />
      <ToolProgress tools={tools} />

      <Box borderStyle="single" borderColor="gray" padding={1} marginTop={1}>
        <MarkdownText content={markdown} />
      </Box>
    </Box>
  );
};
