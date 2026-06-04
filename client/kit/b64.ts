export function base64EncodeString(input: string): string {
  const encoder = new TextEncoder();
  const botCodeBytes = encoder.encode(input);
  const botCodeUtf8 = String.fromCodePoint(...botCodeBytes);
  return btoa(botCodeUtf8);
}

export function decode() {}
