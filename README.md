# extra_lints
more lints for rust

This project aims to collect/add lints to rust, to help check for harmful or questionable idioms.

The first lint is ready:

# eq_op

Checks for equal expressions on both sides of certain binary operations (comparisons, and/or as well as
bitwise and/or/xor). E.g. 1 == 1. This almost always is a programmer error. It works across all operators,
calls, casts, etc., only macro expansions are not yet inspected. It even works for inversions of 
commutative operators, e.g. it will warn on 1 + 2 < 2 + 1. However, it does not use symbolic execution, so
1 + 2 == 3 is not caught.

More ideas (lifted from [FindBugs](http://findbugs.sf.net)):

* Incorrect combination of max / min

TODO:

* Integrate with compilechk
