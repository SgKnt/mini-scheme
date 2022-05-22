(define (fact1 n)
  (if (= n 0)
      1
      (* n (fact1 (- n 1)))
  )
)

(define (fact2 n)
  (let fact2-i ((n n) (acc 1))
    (if (= n 0) 
        acc
        (fact2-i (- n 1) (* acc n))
    )
  )
)

(define (fact3 n)
  (let fact3-i ((n n) (cont (lambda (n) n)))
    (if (= n 0)
        (cont 1)
        (fact3-i (- n 1) (lambda (x) (* x (cont  n))))
    )
  )
)
