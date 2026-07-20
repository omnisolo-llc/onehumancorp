import React, { useEffect } from 'react';
import { Box, Text } from 'ink';

export interface ErrorStateProps {
  error: string;
}

export const ErrorState: React.FC<ErrorStateProps> = ({ error }) => {
  useEffect(() => {
    console.error(error);
  }, [error]);

  return (
    <Box paddingY={1} flexDirection="column">
      <Text bold color="red">ERROR:</Text>
      <Box paddingLeft={2}>
        <Text color="red">{error}</Text>
      </Box>
    </Box>
  );
};
