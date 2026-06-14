export function hey(message: string): string {
  message = message.trim();
  const shouting = isUpperCase(message);

  if (message.endsWith("?")) {
    return shouting ? "Calm down, I know what I'm doing!" : "Sure.";
  }
  if (message === "") {
    return "Fine. Be that way!";
  }
  return shouting ? "Whoa, chill out!" : "Whatever.";
}

function isUpperCase(message: string): boolean {
  return /[A-Z]/.test(message) && !/[a-z]/.test(message);
}
