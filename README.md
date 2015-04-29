# extra_lints
more lints for rust

This project aims to collect/add lints to rust, to help check for harmful or questionable idioms.

Lints that show problematic, but potentially valid idioms should warn, whereas lints that match code which is
certainly erroneous should be deny. Note that this doesn't relate to the safety of the code in question, but hinges on
the certainty with which the code is identified as erroneous.

This project is under MIT License.

## eq_op

Checks for equal expressions on both sides of certain binary operations (comparisons, and/or as well as
bitwise and/or/xor). E.g. 1 == 1. This almost always is a programmer error. It works across all operators,
calls, casts, etc., only macro expansions are not yet inspected.

This lint is **warn** by default

## bad_bit_mask

Checks for incompatible bit masks in comparisons, e.g. `x & 1 == 2`. This cannot work because the bit that makes up
the value two was zeroed out by the bit-and with 1. So the formula for detecting if an expression of the type 
`_ <bit_op> m <cmp_op> c` (where `<bit_op>` is one of {`&`, '|'} and `<cmp_op>` is one of {`!=`, `>=`, `>` ,`!=`, `>=`, 
`>`}) can be determined from the following table:

|Comparison  |Bit-Op|Example     |is always|Formula               |
|------------|------|------------|---------|----------------------|
|`==` or `!=`| `&`  |`x & 2 == 3`|`false`  |`c & m != c`          |
|`<`  or `>=`| `&`  |`x & 2 < 3` |`true`   |`m < c`               |
|`>`  or `<=`| `&`  |`x & 1 > 1` |`false`  |`m <= c`              |
|`==` or `!=`| `|`  |`x | 1 == 0`|`false`  |`c | m != c`          |
|`<`  or `>=`| `|`  |`x | 1 < 1` |`false`  |`m >= c`			  |
|`<=` or `>` | `|`  |`x | 1 > 0` |`true`   |`m > c`               |

*TODO*: There is the open question if things like `x | 1 > 1` should be caught by this lint, because it is basically
an obfuscated version of `x > 1`.

This lint is **deny** by default

## Tasks:

- [ ] Document the code
- [X] Check for equal expressions on both sides of certain operators
- [X] Check for bad bit masks (like [FindBugs' BIT_AND](http://findbugs.sourceforge.net/bugDescriptions.html#BIT_AND), and some other patterns combined)
- [ ] Incorrect combination of max / min (like [FindBugs' DM_INVALID_MIN_MAX](http://findbugs.sourceforge.net/bugDescriptions.html#DM_INVALID_MIN_MAX))

if you want to propose something, feel free to create an issue
