# `scum`

It's about time I made a Lisp of some sort…


# TODO

Note that this isn't currently correct; for instance:

```
λ>  (define scale-by (lambda (n) (lambda (x) (* n x))))
(lambda (n) (lambda (x) (* n x)))
λ>  (define double (scale-by 2))
(lambda (x) (* n x))
λ>  (double 2)
Evaluation error: Unknown identifier n
```

Lambdas need to store their environment for lexical captures.
