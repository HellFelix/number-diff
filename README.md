 # Overview
 ## Number Diff - An all-purpose tool for calculus

 ### Functions
 Number Diff is built around a calculus-like function, that is, a function that takes an
 f64 as an argument, returning an f64 according to some specific rule. In the current state of
 the crate, functions are limited to ƒ: ℝ ⟶ ℝ (have a look at the [supported
 functions](#supported-functions) for which functions can be used).  
 There are plans to expand to ƒ: ℂ ⟶ ℂ in the not so distant future.

 #### Usage
 Functions are represented by the Function struct. The Function struct can be created by either
 parsing a string or combining functions using standard operations. A Function
 instance can then be used with the call(x) method (or when using the nightly feature, Function
 instances can be called directly).

 Check out [some examples](https://github.com/HellFelix/number-diff/tree/main/examples)!

 ## Supported functions
 | Function | Parsing Identifier | In-code Function |
 |:----------:|:-------------------:|:-----------------:|
 | sin      | "sin(_)"            | sin()|
 | cos       | "cos(_)"            | cos()|
 | tan       | "tan(_)"            | tan()|
 | sec       | "sec(_)"            | sec()|
 | csc       | "csc(_)"            | csc()|
 | cot       | "cot(_)"            | cot()|
 | asin      | "asin(_)"            | asin()|
 | acos      | "acos(_)"            | acos()|
 | atan      | "atan(_)"            | atan()|
 | sinh      | "sinh(_)"            | sinh()|
 | cosh      | "cosh(_)"            | cosh()|
 | tanh      | "tanh(_)"            | tanh()|
 | natural log|"ln(_)"            | ln()|
 | absolute value|"abs(_)"           | abs()|
 | square root|"cosh(_)"            | cosh()|
 | factorial | "_!"              | factorial()|
 | addition | "_ + _ "            | +|
 | subtraction| "_ - _"            | -|
 | multiplication| "_ * _"       | *|
 | division   | "_ / _"           | /|
 | contant | "1", "-12", "3.14", etc. | f64|
 | independent variable | "x"    | Function::default()|

 Note that "_" in the table above refers to any other function of the ones provided above. Note also that the
 operations (+, -, *, /) cannot be applied to each other. Attempting to apply an operation to
 another operation will make the parser return a Parsing Error.

 ### Derivatives
 All of the [supported functions](#supported-functions) are [smooth functions](https://en.wikipedia.org/wiki/Smoothness) which in turn
 means that once initialized, a Function is guaranteed to be a smooth function and so are all of its
 derivatives.

 Derivatives are calculated analytically. The provided derivative function will always be the
 the exact derivative of the original function (although not always in simplest form).

 Note that in its current state, differentiating might in some rare cases return NaN for
 certain input values where simplification fails to avoid a division by zero.

 Function instances can be differentiated using the differentiate()
 method or using the derivative_of() function.

 ### Integrals
 Integration is stable for the most part. With a standard precision of 1000, integration uses
 Simpson's rule in order to find an approximate value of the integral.

 For usage examples, check out the integration documentation!

 Note that while integrating over an interval (including the bounds of integration) inside of which the value of the
 specified function is undefined, the resulting value might be NaN.

 Also, integrating over an interval (including the bounds of integration) inside of which the value of the
 specified function is infinit, the resulting value might be inf even though the integral should converge.

 ### Series Expansions
 See [this article](https://en.wikipedia.org/wiki/Series_expansion) for an explanation of series
 expansions.

 #### Current stability of series expansions

 | Expansion Technique | Stability | Usage       |
 |:-------------------:|:---------:|:-----------:|
 | Taylor series       | Stable ✅ | get_taylor_expansion()|
 | Maclaurin series    | Stable ✅ | get_maclaurin_expansion()|
 | Fourier series      | Unimplemented ❌| N/A |
