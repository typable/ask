# ask
A minimalistic programming language
<br>
<br>

## Installation

Use the following commands to install the CLI:

```
git clone https://github.com/typable/ask.git
cd ask
cargo install --path .
```
Use the "ask" command to execute `.ask` files.
```
ask <file>
```
<br>

## Syntax

### Operations

An operation can have the following argument types:

- `pos`: Variable name
- `val`: Value (only numbers allowed)
- `label`: Pin name

The following operations are currently defined:

|Operation|Description|
|---|---|
|`mov [pos] [pos\|val]`|Moves value into position|
|`add [pos] [pos\|val]`|Adds the second argument to the first and stores the result in the first position|
|`sub [pos] [pos\|val]`|Subtracts the second argument from the first and stores the result in the first position|
|`cmp [pos] [pos\|val]`|Compares to values. The result is passed to the following operation|
|`jif [label]`|Jumps to pin if previous cmp operation was 1|
|`jel [label]`|Jumps to pin if previous cmp operation was 0|
|`jmp [label]`|Jumps to pin|
|`out [pos\|val]`|Prints the value|
|`utf [pos\|val]`|Prints the value as UTF-8 character|
|`ret`|Jumps back to the calling jump operation|
|`end`|Exits the program|
<br>

### Pins

Loops are accomplished by placeing "pins" and jumping back to them with a jump operation.<br>
Pins are written in the following syntax `:[label]` and the label name must be unique.

Example:
```
mov n 0
:loop
  add n 1
  cmp n 3
  jel loop
out n
```
Increase pos "n" by 1 as long as it is less than 3.
<br>
<br>

### Comments

Use the `"` symbol at the start of the line to mark it as a comment.

Example:
```
" store newline char code
mov newline 10
```
<br>

### Casting

Use the `&` symbol to cast a character into a number.

Example:
```
" store letter a
mov letter_a = &'a'
```
