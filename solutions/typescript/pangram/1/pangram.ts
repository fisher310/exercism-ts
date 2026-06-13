export function isPangram(sentence: string): boolean {
  if (sentence.length === 0) {
    return false;
  }
  return (
    new Set(
      sentence
        .toLowerCase()
        .split("")
        .filter((c) => c >= "a" && c <= "z"),
    ).size === 26
  );
}
