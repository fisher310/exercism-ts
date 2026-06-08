export function toRna(dna: string): string | never {
  
  const rna = dna.split('').map((nucleotide) => {
    switch (nucleotide) {
      case 'A':
        return 'U'
      case 'C':
        return 'G'
      case 'G':
        return 'C'
      case 'T':
        return 'A'
      default:
        throw new Error('Invalid input DNA.')
    }
  }).join('')
  return rna
}
