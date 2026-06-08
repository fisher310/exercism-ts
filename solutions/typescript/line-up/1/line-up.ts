export function format(name: string, number: number): string {
    const last = number % 10;
    const lastTwo = number % 100;
  const suffix =
    last === 1 && lastTwo !== 11
      ? "st"
      : last === 2 && lastTwo !== 12
        ? "nd"
        : last === 3 && lastTwo !== 13
          ? "rd"
          : "th";
  return `${name}, you are the ${number}${suffix} customer we serve today. Thank you!`
}
