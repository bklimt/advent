
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
    (list (parse-list (subseq s 0 i))
          (parse-item (subseq s (+ i 4))))))

(defun read-lines (f)
  (let ((line (read-line f nil)))
    (if line
      (cons (parse-equation line) (read-lines f)))))

(defun read-file (path)
  (let* ((f (open path))
         (lines (read-lines f)))
    (close f)
    lines))

(setq rules (read-file "input1.txt"))
