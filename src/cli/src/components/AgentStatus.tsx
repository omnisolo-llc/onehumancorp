import React from 'react';
import { Text, Box } from 'ink';
import Spinner from 'ink-spinner';

export interface AgentStatusProps {
  status: string;
}

export const AgentStatus: React.FC<AgentStatusProps> = ({ status }) => {
  return (
    <Box paddingY={1}>
      <Text color="cyan">
        <Spinner type="dots" />
      </Text>
      <Text color="white"> {status}</Text>
    </Box>
  );
};
