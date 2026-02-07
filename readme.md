###
---
###

## `# crate information`

### `purpose` - provide high quality, portable, `no_std` compatible low level primitives with rigorous proofs

### `proofs` - 
* ### `creusot-contracts` - (now `creusot-std`) for integer proofs
* ### `kani` - for binary operation boundary proofs

###
---
###

## `# changes`

### `problem` - this crate was intended to contain the proof logic

### `realization` - nightly and unstable features needed for creusot would compromise the pure and discrete nature of this library for the dependents


###
---
###

## `# the new plan`
### we splt each concern by purpose
###
*    ### `honest` - pure logic
*    ### `honest_proof` - `creusot-contracts` / `creusot-std`
*    ### `honest_harness` - the kani 
### *

###
---
###