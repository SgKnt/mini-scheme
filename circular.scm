(define l (list 1 2 3 4))
(set-cdr! (cdr (cdr l)) l)
(display l)
