
(defun parse-item (s)
  (let ((i (search " " s)))
    (list (parse-integer (subseq s 0 i))
          (intern (subseq s (+ i 1))))))

(defun parse-list (s)
  (let ((i (search ", " s)))
    (if (null i)
        (list (parse-item s))
        (cons (parse-item (subseq s 0 i))
              (parse-list (subseq s (+ i 2)))))))

(defun parse-equation (s)
  (let ((i (search " => " s)))
    (list (parse-item (subseq s (+ i 4)))
          (parse-list (subseq s 0 i)))))

(defun read-lines (f)
  (let ((line (read-line f nil)))
    (if line
      (cons (parse-equation line) (read-lines f)))))

(defun read-file (path)
  (let* ((f (open path))
         (lines (read-lines f)))
    (close f)
    lines))

(defun count-ore-for-element (element rules)
  ; (format t "searching for: ~A in rules: ~A~%" element rules)
  (if rules
      ; Check the first rule.
      (let* ((name (cadr element))
             (amount (car element))
             (rule (car rules))
             (lhs (car rule))
             (rule-name (cadr lhs))
             (rule-amount (car lhs)))
            (format t "checking for: ~A in rule: ~A~%" element rule)
            (if (eq rule-name name)
              (format t "   YES!~%"))
            ; Check the rest of the rules.
            (count-ore-for-element element (cdr rules)))))

(defun count-ore-for-elements (elements rules)
  (if elements
      (cons (count-ore-for-element (car elements) rules)
            (count-ore-for-elements (cdr elements) rules))))

(setq rules (read-file "input1.txt"))

(setq ore (count-ore-for-elements '((1 FUEL)) rules))
