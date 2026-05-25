import React from 'react';
import { Text } from 'ink';
export default function TextInput({ value, onChange, placeholder }: any) {
  return <Text>{value || placeholder}</Text>;
}
