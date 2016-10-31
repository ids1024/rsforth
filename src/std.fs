: cr 10 emit ;
: space 32 emit ;
: 0= 0 = ;
: 0< 0 < ;
: 0> 0 > ;
: ?dup dup 0= if dup then ;
: max over over < if swap then drop ;
: min over over > if swap then drop ;
: negate 0 swap - ;
: false 0 ;
: true -1 ;
variable base
10 base ! ( not used yet )
: +! dup @ rot + swap ! ;
