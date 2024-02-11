Infinite sequences of numbers, primarily focussed on the OEIS.

For now implementing out of interest and learning, some things might be useful.
More likely a specialized version of these sequences are more useful.

For example: the primes sequence is a reasonably efficient implementation of
the Sieve of Eratosthenes and computes the primes lazily, but if you are looking
for something truly optimized go for `primesieve` a rust binding to the primesieve
algorithm or `primal` a pure rust re-implementation of primesieve.
