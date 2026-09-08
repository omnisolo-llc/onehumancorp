import React from 'react';
import { Box, Text } from 'ink';

export const Header: React.FC = () => {
  return (
    <Box justifyContent="center" marginBottom={1} borderStyle="double" borderColor="cyan" padding={1}>
      <Text bold color="white" backgroundColor="blue"> OMNISOLO </Text>
      <Text dimColor> - Standalone Agent Mode </Text>
    </Box>
  );
};
