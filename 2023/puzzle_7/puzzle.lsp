(defconstant *puzzle-7-example* "puzzle_7/example.txt")
(defconstant *puzzle-7-input* "puzzle_7/input.txt")

(defvar *card-values* (make-hash-table :test 'equal))
(loop for char across "23456789TJQKA" for i from 2 do
  (setf (gethash (string char) *card-values*) i))


(defvar *card-values-part-2* (make-hash-table :test 'equal))
(loop for char across "J23456789TQKA" for i from 1 do
  (setf (gethash (string char) *card-values-part-2*) i))

(defun split-string (string)
    (loop with start = 0
        for pos = (position #\Space string :start start)
        collect (subseq string start (or pos (length string)))
        do (setf start (when pos (+ pos 1)))
        while pos))

(defun count-chars (str)
    (let ((table (make-hash-table :test 'equal)))
        (loop for char across str do
            (incf (gethash char table 0)))
        table))

(defun num-distinct-chars (table)
    (hash-table-count table))

(defun max-char-count (table)
    (loop for char being the hash-keys of table using (hash-value count)
        maximize count))

; Order hands via an 11 digit number, first one for the hand type, then pairs
; of digits for the card values
(defun assign-hand-type-value (counts)
    (let* (
        (num-distinct (num-distinct-chars counts))
        (max-count (max-char-count counts)))
        (cond
            ((= num-distinct 1) (* 7 (expt 10 10)))  ; Five of a kind
            ((and (= num-distinct 2) (= max-count 4)) (* 6 (expt 10 10)))  ; Four of a kind
            ((and (= num-distinct 2) (= max-count 3)) (* 5 (expt 10 10)))  ; Full house
            ((and (= num-distinct 3) (= max-count 3)) (* 4 (expt 10 10)))  ; Three of a kind
            ((and (= num-distinct 3) (= max-count 2)) (* 3 (expt 10 10)))  ; Two pair
            ((= num-distinct 4) (* 2 (expt 10 10)))  ; One pair
            (t (expt 10 10)))))

(defun print-counts (counts)
    (loop for char being the hash-keys of counts using (hash-value count)
        do (format t "~a: ~a~%" char count)))

(defun get-max-occurring-non-j-char (hash-table)
  (loop with max-value = nil
        with max-key = nil
        for key being the hash-keys of hash-table using (hash-value value)
        do (when (and (not (char= key #\J)) (or (not max-value) (> value max-value)))
             (setf max-value value)
             (setf max-key key))
        finally (return max-key)))

(defun assign-hand-type-value-part-2 (counts)
    (let* (
        (num-distinct-initial (num-distinct-chars counts))
        (max-count-initial (max-char-count counts)))
        ; if \#J is in the hand and num-distinct > 1, find the character with the
        ; highest count and assign J's count to it, then delete J's count
        (cond 
            ((and (> (gethash #\J counts 0) 0) (> num-distinct-initial 1))
                (incf (gethash (get-max-occurring-non-j-char counts) counts) (gethash #\J counts))
                (remhash #\J counts)))
        (let* ((num-distinct (num-distinct-chars counts))
               (max-count (max-char-count counts)))
        (assign-hand-type-value counts))))

(defun get-card-values (cards values)
    (reduce #'+
            (loop for card across cards for i from 0 collect
                (* (expt 10 (- 8 (* i 2))) (gethash (string card) values)))))

(defun get-total-hand-value (line)
    (let* ((parts (split-string line))
           (cards (first parts))
           (raw-bid (second parts))
           (bid (parse-integer raw-bid))
           (counts (count-chars cards)))
           (vector (+ (assign-hand-type-value counts) (get-card-values cards *card-values*)) bid)))

(defun get-total-hand-value-part-2 (line)
    (let* ((parts (split-string line))
           (cards (first parts))
           (raw-bid (second parts))
           (bid (parse-integer raw-bid))
           (counts (count-chars cards)))
           (vector (+ (assign-hand-type-value-part-2 counts) (get-card-values cards *card-values-part-2*)) bid)))

(defun get-final-answer (ordered-hands)
    (loop for hand in ordered-hands
        for i from 1
        sum (* i (aref hand 1))))

(defun puzzle-7-part-1 (path)
    (with-open-file (stream path)
        (let ((ordered-hands
            (loop for line = (read-line stream nil)
                while line
                collect (get-total-hand-value line))))
            (setq ordered-hands (sort ordered-hands #'< :key #'(lambda (v) (aref v 0))))
            (format t "~a~%" (get-final-answer ordered-hands)))))

(defun puzzle-7-part-2 (path)
    (with-open-file (stream path)
        (let ((ordered-hands
            (loop for line = (read-line stream nil)
                while line
                collect (get-total-hand-value-part-2 line))))
            (setq ordered-hands (sort ordered-hands #'< :key #'(lambda (v) (aref v 0))))
            (format t "~a~%" (get-final-answer ordered-hands)))))

(puzzle-7-part-1 *puzzle-7-example*)
(puzzle-7-part-1 *puzzle-7-input*)
(puzzle-7-part-2 *puzzle-7-example*)
(puzzle-7-part-2 *puzzle-7-input*)
        
