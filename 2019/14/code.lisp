
;; Parses an expression like "1 A" to (A . 1).
(defun parse-item (s)
  (let ((i (search " " s)))
    (cons (intern (subseq s (+ i 1)))
          (parse-integer (subseq s 0 i)))))

;; Parses a list like "1 A, 2 B" to ((A . 1)(B . 2)).
(defun parse-list (s)
  (let ((i (search ", " s)))
    (if (null i)
        (list (parse-item s))
        (cons (parse-item (subseq s 0 i))
              (parse-list (subseq s (+ i 2)))))))

;; Parses an equation like "1 A, 2 B => 3 C" to ((C . 3)(A . 1)(B . 2)).
(defun parse-equation (s)
  (let ((i (search " => " s)))
    (cons (parse-item (subseq s (+ i 4)))
          (parse-list (subseq s 0 i)))))

;; Parses a series of equations from a file and returns the list.
(defun parse-file (f)
  (let ((line (read-line f nil)))
    (if line
      (cons (parse-equation line) (parse-file f)))))

;; Parses the equations from a path and returns the list.
(defun read-file (path)
  (let* ((f (open path))
         (equations (parse-file f)))
    (close f)
    equations))

;; Inserts or increments the item in the a-list lst by the given amount.
(defun assoc-inc (item amount lst)
  (cond ((null lst)
         (list (cons item amount)))
        ((eq item (caar lst))
         (cons (cons item (+ amount (cdar lst))) (cdr lst)))
        (t (cons (car lst)
                 (assoc-inc item amount (cdr lst))))))

;; Remove the item from an a-list.
(defun assoc-rm (item lst)
  (cond ((null lst) nil)
        ((eq item (caar lst)) (cdr lst))
        (t (cons (car lst) (assoc-rm item (cdr lst))))))

;; Takes the alist with amounts of items, and maybe applies the given rule.
;; (apply-rule '((FUEL . 3)(A . 1)) '((FUEL . 1)(A . 2)(B . 3)))
;; -> '((A . 7)(B . 9))
(defun apply-rule (inventory rule)
  (let* ((lhs (car rule))
         (rhs (cdr rule))
         (rule-element (car lhs))
         (rule-amount (cdr lhs))
         (inv-entry (assoc rule-element inventory))
         (inv-amount (cdr inv-entry)))
    (if inv-entry
        ;; The rule is relevant. Figure out how many times to apply it.
        (let* ((factor (ceiling inv-amount rule-amount))
               ;; Multiply all the items in the lhs by the given amount.
               (factored-rule (mapcar (lambda (cell)
                                        (cons (car cell)
                                              (* factor (cdr cell))))
                                      rhs))
               (added-inv (reduce (lambda (inv rule-entry)
                                    (assoc-inc (car rule-entry)
                                               (cdr rule-entry) inv))
                                  factored-rule
                                  :initial-value inventory))
               (new-inv (assoc-rm rule-element added-inv)))
          new-inv))))

;; Returns the count for ORE if the inventory only contains ORE.
(defun amount-if-only-ore (inventory)
  (cond ((cdr inventory) nil)                  ; Inventory has multiple items.
        ((not (eq 'ORE (caar inventory))) nil) ; Item isn't ORE.
        (t (cdar inventory))))

(defun apply-rules-int (inventory remaining-rules all-rules prefix)
  (let ((rule (car remaining-rules))
        (inner-prefix (format nil " ~A" prefix))
        (ore-count (amount-if-only-ore inventory)))
    (cond ((not (null ore-count))              ; We're done.
           (format t "~A ORE = ~A~%" prefix ore-count)
           ore-count)
          (remaining-rules                     ; There are still more rules.
           (let ((ore-count-1 (apply-rules-int inventory
                                               (cdr remaining-rules)
                                               all-rules
                                               prefix))
                 (new-inv (apply-rule inventory rule)))
             (cond (new-inv
                    (format t "~A  Got new inventory ~A from rule ~A~%"
			    prefix new-inv rule)
                    (let ((ore-count-2 (apply-rules-int new-inv
                                                        all-rules
                                                        all-rules
                                                        inner-prefix)))
                      ;; (format t "~A  Returning min(~A, ~A)~%"
                      ;;         prefix ore-count-1 ore-count-2)
                      (cond ((null ore-count-2) ore-count-1)
                            ((null ore-count-1) ore-count-2)
                            ((< ore-count-1 ore-count-2) ore-count-1)
                            (t ore-count-2))))
                   (t                          ; This rule didn't do anything.
                    ;; (format t "~A  Rule doesn't apply. Returning ~A~%"
                    ;;         prefix ore-count-1)
                    ore-count-1)))))))

(defun apply-rules (inventory rules)
  (apply-rules-int inventory rules rules ""))

(setq rules (read-file "input1.txt"))

; (setq ore (count-ore-for-elements '((1 FUEL)) rules))
