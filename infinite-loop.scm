(define (foo) (bar))
(define (bar) (baz))
(define (baz)
  (let ((l (list 1 2 3)))
    "just make circular list"
    (set-cdr! (cdr l) l))
  (foo)
)
(foo)
