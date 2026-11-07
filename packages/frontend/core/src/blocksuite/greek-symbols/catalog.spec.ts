import { describe, expect, it } from 'vitest';

import {
  findGreekLetter,
  greekLetters,
  resolveGreekSymbol,
} from './catalog';

describe('Greek symbol catalogue', () => {
  it('contains all 24 standard Greek letters', () => {
    expect(greekLetters).toHaveLength(24);
  });

  it('resolves canonical names and slash-command names', () => {
    expect(findGreekLetter('alpha')?.lowercase).toBe('α');
    expect(findGreekLetter('/theta')?.uppercase).toBe('Θ');
    expect(resolveGreekSymbol('/pi')).toBe('π');
  });

  it('resolves documented aliases without confusing case', () => {
    expect(resolveGreekSymbol('th')).toBe('θ');
    expect(resolveGreekSymbol('ps')).toBe('ψ');
    expect(resolveGreekSymbol('ALPHA', 'uppercase')).toBe('Α');
  });

  it('returns no result for empty or unknown input', () => {
    expect(findGreekLetter('')).toBeUndefined();
    expect(findGreekLetter('not-a-letter')).toBeUndefined();
    expect(resolveGreekSymbol('/not-a-letter')).toBeUndefined();
  });

  it('trims whitespace and accepts an optional leading slash', () => {
    expect(resolveGreekSymbol('  /omega  ')).toBe('ω');
  });
});
