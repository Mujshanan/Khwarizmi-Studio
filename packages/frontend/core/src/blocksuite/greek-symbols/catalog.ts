/**
 * Shared Greek-symbol catalogue for Khwarizmi editor insertion.
 *
 * This module contains presentation/input metadata only. It does not assign
 * engineering or calculation semantics to a symbol.
 */
export type GreekLetter = {
  /** Canonical English name used by slash commands. */
  name: string;
  /** Lowercase Unicode character. */
  lowercase: string;
  /** Uppercase Unicode character. */
  uppercase: string;
  /** Search terms accepted by the picker and slash-command resolver. */
  aliases: readonly string[];
};

export const greekLetters: readonly GreekLetter[] = [
  { name: 'alpha', lowercase: 'α', uppercase: 'Α', aliases: ['a'] },
  { name: 'beta', lowercase: 'β', uppercase: 'Β', aliases: ['b'] },
  { name: 'gamma', lowercase: 'γ', uppercase: 'Γ', aliases: ['g'] },
  { name: 'delta', lowercase: 'δ', uppercase: 'Δ', aliases: ['d'] },
  { name: 'epsilon', lowercase: 'ε', uppercase: 'Ε', aliases: ['e', 'varepsilon'] },
  { name: 'zeta', lowercase: 'ζ', uppercase: 'Ζ', aliases: ['z'] },
  { name: 'eta', lowercase: 'η', uppercase: 'Η', aliases: ['h'] },
  { name: 'theta', lowercase: 'θ', uppercase: 'Θ', aliases: ['th', 'vartheta'] },
  { name: 'iota', lowercase: 'ι', uppercase: 'Ι', aliases: ['i'] },
  { name: 'kappa', lowercase: 'κ', uppercase: 'Κ', aliases: ['k'] },
  { name: 'lambda', lowercase: 'λ', uppercase: 'Λ', aliases: ['l'] },
  { name: 'mu', lowercase: 'μ', uppercase: 'Μ', aliases: ['m'] },
  { name: 'nu', lowercase: 'ν', uppercase: 'Ν', aliases: ['n'] },
  { name: 'xi', lowercase: 'ξ', uppercase: 'Ξ', aliases: ['x'] },
  { name: 'omicron', lowercase: 'ο', uppercase: 'Ο', aliases: ['o'] },
  { name: 'pi', lowercase: 'π', uppercase: 'Π', aliases: ['p'] },
  { name: 'rho', lowercase: 'ρ', uppercase: 'Ρ', aliases: ['r', 'varrho'] },
  { name: 'sigma', lowercase: 'σ', uppercase: 'Σ', aliases: ['s', 'varsigma'] },
  { name: 'tau', lowercase: 'τ', uppercase: 'Τ', aliases: ['t'] },
  { name: 'upsilon', lowercase: 'υ', uppercase: 'Υ', aliases: ['u'] },
  { name: 'phi', lowercase: 'φ', uppercase: 'Φ', aliases: ['f', 'varphi'] },
  { name: 'chi', lowercase: 'χ', uppercase: 'Χ', aliases: ['ch'] },
  { name: 'psi', lowercase: 'ψ', uppercase: 'Ψ', aliases: ['ps'] },
  { name: 'omega', lowercase: 'ω', uppercase: 'Ω', aliases: ['w'] },
] as const;

export type GreekLetterVariant = 'lowercase' | 'uppercase';

/** Resolve a letter by its canonical name or alias, case-insensitively. */
export function findGreekLetter(query: string): GreekLetter | undefined {
  const normalized = query.trim().replace(/^\//u, '').toLocaleLowerCase();
  if (!normalized) return undefined;

  return greekLetters.find(
    letter =>
      letter.name === normalized ||
      letter.aliases.some(alias => alias.toLocaleLowerCase() === normalized)
  );
}

/** Resolve a slash-command name or alias to its Unicode character. */
export function resolveGreekSymbol(
  query: string,
  variant: GreekLetterVariant = 'lowercase'
): string | undefined {
  const letter = findGreekLetter(query);
  return letter?.[variant];
}
