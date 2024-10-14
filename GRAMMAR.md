<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" 
      srcset="https://github.com/user-attachments/assets/bfd2d883-6689-4593-b8b5-5ea6e09d9116">
    <source media="(prefers-color-scheme: light)"
      srcset="https://github.com/user-attachments/assets/bfd2d883-6689-4593-b8b5-5ea6e09d9116">
    <img alt="GRAMMAR"
      src="https://github.com/user-attachments/assets/bfd2d883-6689-4593-b8b5-5ea6e09d9116"
      width="69%">
  </picture>
</div>


<div align="center">
  <h1><i>⸨</i> The Beginning <i>⸩</i></h1>
  <i><b> Note: </b> Files should use the .wcl extension. </i>
</div>


<div align="center">
  <h2> Currency </h2>
</div>

Make sure to specify your **currency** at the top of the file, using `!currency`. This is required to make sure fellow programmers from around the world can understand their code. Totally. Yup.

Examples of setting currency (Your currency **must** be a valid currency sign, otherwise the code will fuck up):
```
!currency $
```
```
!currency ¥
```
```
!currency £
```
Throughout this guide, we will be using the dollar sign ($) as the currency in our examples.


<div align="center">
  <h1><i>⸨</i> Types <i>⸩</i></h1>
  <i><b> Note: </b> Types must be enclosed in arrows, eg. <code>&lt;type&gt;</code>. </i>
</div>


<div align="center">
  <h2> Integer </h2>
</div>

**Integers** (denoted by `integer`) are whole numbers, meaning they have no fractional components. Their values can be negative, zero, or positive.

**Unsigned integers** (denoted by `unsigned-integer`) are whole numbers like integers, however, they cannot be negative.

### Integer types

| Type                      | Size (Bytes) | Size (Minimum / Maximum value)
| :---                      | :---         | :---
| `integer-8-bit`           | 1            | -128 / 127
| `integer-16-bit`          | 2            | -32,768 / 32,767
| `integer-32-bit`          | 4            | -2,147,483,648 / 2,147,483,647
| `integer-64-bit`          | 8            | -9,223,372,036,854,775,808 / 9,223,372,036,854,775,807
| `unsigned-integer-8-bit`  | 1            | 0 / 255
| `unsigned-integer-16-bit` | 2            | 0 / 65,535
| `unsigned-integer-32-bit` | 4            | 0 / 4,294,967,295
| `unsigned-integer-64-bit` | 8            | 0 / 18,446,744,073,709,551,615


<div align="center">
  <h2> Boolean </h2>
</div>

Booleans can either be `true` or `untrue`. *It's as shrimple as that!*


<div align="center">
  <h1><i>⸨</i> Functions <i>⸩</i></h1>
  <i><b> Note: </b> Functions must be enclosed in your currency's symbol, eg. <code>$function$</code>. </i>
</div>


<div align="center">
  <h2> Declaring functions </h2>
</div>

wip, call the function `foobify` in example


<div align="center">
  <h2> Calling functions </h2>
</div>

Calling functions is somewhat straight-forward. Here's how we can call our function `foobify`:
```
$ $foobify$ {[foo] then [bar] then [baz]}
```
Here, we first add a `$` to indicate that we are calling the function, then type our function name `$foobify$`. Then, we


<div align="center">
  <h1><i>⸨</i> Variables <i>⸩</i></h1>
  <i><b> Note: </b> Variables must be enclosed in square brackets, eg. <code>[variable]</code>. </i>
</div>


<div align="center">
  <h2> Declaring and setting variables </h2>
</div>

To declare variables, use the `declare-vars` function:
```
$ $declare-vars$ {[var1] as <type> then [var2] as <type>}
```
We first specify the variable name with `[var]`, then we specify the type of variable with `as <type>`. We set two variables here, by using `then`. We can declare as many variables as we want in one line! Handy, yeah?

You must then set the variables in a seperate line using `set-vars`:
```
$ $set-vars$ {[var1] to (value) then [var2] to (value)}
```
In a similar fashion to declare-var, we specify the variable names with `[var]`, and we set them to a value of the correct type with `to (value)`. We can, again, use `then` to set two variables.

**Variables *must* be set in the same way as they are declared.** If you want to set two variables at different lines in the code, you must both declare and set them in seperate lines.
