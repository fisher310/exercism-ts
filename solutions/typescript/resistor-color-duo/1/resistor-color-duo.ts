const COLORS = [
  "black",
  "brown",
  "red",
  "orange",
  "yellow",
  "green",
  "blue",
  "violet",
  "grey",
  "white",
];

export function decodedValue(colors: string[]): number {
  const firstColor = colors[0];
  const secondColor = colors[1];
  return COLORS.indexOf(firstColor) * 10 + COLORS.indexOf(secondColor);
}
