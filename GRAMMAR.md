# GRAMMAR

### Files
Files should have the .wcl extension.

### Variables
Declaring a variable is really quite simple actually, here's how you can declare a variable named x.
```
set_vars{[x] to (1)}
```
This will set the value `x` to `1`.  
You can even define multiple variables on one line like this:
```
set_vars{[x] then [y] to (1) then (2)}
```
This will set two variables, `x` and `y`, `x` will have the value of `1` and `y` will have the value of `2`.
