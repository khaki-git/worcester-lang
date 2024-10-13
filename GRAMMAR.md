<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/bfd2d883-6689-4593-b8b5-5ea6e09d9116">
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/bfd2d883-6689-4593-b8b5-5ea6e09d9116">
    <img alt="GRAMMAR"
         src="https://github.com/user-attachments/assets/bfd2d883-6689-4593-b8b5-5ea6e09d9116"
         width="60%">
  </picture>
</div>


<div align="center">
  <h1>The Beginning</h1>
  <i><b>Note:</b> Files should use the .wcl extension. </i>
</div>


<div align="center">
  <h3>Currency</h3>
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
  <h1>Types</h1>
</div>

Types must be enclosed in arrows, eg. `<type>`.

wip


<div align="center">
  <h1>Functions</h1>
</div>

Functions must be enclosed in your currency's symbol, eg. `$function$`.


<div align="center">
  <h3>Declaring functions</h3>
</div>

wip, call the function `foobify` in example


<div align="center">
  <h3>Calling functions</h3>
</div>

Calling functions is somewhat straight-forward. Here's how we can call our function `foobify`:
```
$$foobify$ {[foo] then [bar] then [baz]}
```
Here, we first add a `$` to indicate that we are calling the function, then type our function name `$foobify$`. Then, we


<div align="center">
  <h1>Variables</h1>
</div>

Variables must be enclosed in square brackets, eg. `[variable]`.


<div align="center">
  <h3>Declaring and setting variables</h3>
</div>

To declare variables, use the `declare-vars` function:
```
$$declare-vars$ {[var1] as <type> then [var2] as <type>}
```
We first specify the variable name with `[var]`, then we specify the type of variable with `as <type>`. We set two variables here, by using `then`. We can declare as many variables as we want in one line! Handy, yeah?

You must then set the variables in a seperate line using `set-vars`:
```
$$set-vars$ {[var1] to (value) then [var2] to (value)}
```
In a similar fashion to declare-var, we specify the variable names with `[var]`, and we set them to a value of the correct type with `to (value)`. We can, again, use `then` to set two variables.

***VARIABLES MUST BE SET IN THE SAME ORDER AS THEY ARE DECLARED!*** If you want to set two variables at different lines in the code, you must both declare and set them in seperate lines.
