import React from 'react';
import { Text, Box } from 'ink';
import Spinner from 'ink-spinner';

export interface AgentStatusProps {
  status: string;
}

export const AgentStatus: React.FC<AgentStatusProps> = ({ status }) => {
  return (
    <Box paddingY={1} paddingX={2} borderStyle="round" borderColor="cyan" dimColor marginBottom={1}>
      <Box marginRight={1}>
        <Text color="cyan" bold>
          <Spinner type="dots" />
        </Text>
      </Box>
      <Text color="white" bold>{status}</Text>
    </Box>
  );
};
