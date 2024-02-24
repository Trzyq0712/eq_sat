# Assumptions about LLVM IR structure

- No Phi instructions
- No early returns
- Only singular loop back-edge
- All loop variables are stack allocated, i.e. everything that gets modified inside
  a loop must be behind a stack pointer, follows from no Phi instructions assumption


