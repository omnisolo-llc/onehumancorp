import React from 'react';
import { Text, Box } from 'ink';
import Spinner from 'ink-spinner';

export interface AgentStatusProps {
  status: string;
  type?: 'loading' | 'success' | 'error';
}

export const AgentStatus: React.FC<AgentStatusProps> = ({ status, type = 'loading' }) => {
  const isError = type === 'error';
  const isSuccess = type === 'success';
  const borderColor = isError ? 'red' : isSuccess ? 'green' : 'cyan';

  return (
    <Box paddingY={1} paddingX={2} borderStyle="round" borderColor={borderColor} dimColor marginBottom={1}>
      <Box marginRight={1}>
        <Text color={borderColor} bold>
          {type === 'loading' ? <Spinner type="dots" /> : isSuccess ? '✓' : '✖'}
        </Text>
      </Box>
      <Text color="white" bold>{status}</Text>
    </Box>
  );
};
