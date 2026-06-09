import { randomInt } from "crypto";

export class DnDCharacter {
  public hitpoints: number;
  public strength: number;
  public dexterity: number;
  public constitution: number;
  public intelligence: number;
  public wisdom: number;
  public charisma: number;

  constructor() {
    this.strength = DnDCharacter.generateAbilityScore();
    this.dexterity = DnDCharacter.generateAbilityScore();
    this.constitution = DnDCharacter.generateAbilityScore();
    this.intelligence = DnDCharacter.generateAbilityScore();
    this.wisdom = DnDCharacter.generateAbilityScore();
    this.charisma = DnDCharacter.generateAbilityScore();
    this.hitpoints = 10 + DnDCharacter.getModifierFor(this.constitution);
  }
  public static generateAbilityScore(): number {
    const a = randomInt(1, 6);
    const b = randomInt(1, 6);
    const c = randomInt(1, 6);
    const d = randomInt(1, 6);
    return DnDCharacter.top3(a, b, c, d);
  }

  private static top3(a: number, b: number, c: number, d: number): number {
    return a + b + c + d - Math.min(a, b, c, d);
  }

  public static getModifierFor(abilityValue: number): number {
    return Math.floor((abilityValue - 10) / 2);
  }
}
