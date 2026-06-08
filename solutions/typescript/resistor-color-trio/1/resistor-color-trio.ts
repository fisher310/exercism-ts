const colorMap: Record<string, number> = {
  'black': 0,
  'brown': 1,
  'red': 2,
  'orange': 3,
  'yellow': 4,
  'green': 5,
  'blue': 6,
  'violet': 7,
  'grey': 8,
  'white': 9,
}
export function decodedResistorValue(colors: string[]): string {
  
  const firstValue = colorMap[colors[0]];
  const secondValue = colorMap[colors[1]];
  const thirdValue = colorMap[colors[2]];

  let value = firstValue * 10 + secondValue;
  
  for (let i = 0; i< thirdValue; i++) {
    value *= 10;
  }
  
  if (value >= 1000000000) {
    return `${value / 1000000000} gigaohms`;
  } else if (value >= 1000000) {
    return `${value / 1000000} megaohms`;
  } else if (value >= 1000) {
    return `${value / 1000} kiloohms`;
  } else {
    return `${value} ohms`;
  }

}
