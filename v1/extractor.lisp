;; Run in SLIME to load packages -
;; ```
;;   (ql:quickload :drakma)
;;   (ql:quickload :parse-float)
;;   (ql:quickload :str)
;; ```

(defun http-get (url)
  "Makes a http GET request to URL and returns nil on failure and the
response body on success."
  (multiple-value-bind (body status)
      (drakma:http-request url)
    (if (eq status 200)
        body
        nil)))

(defun split-string (delim str)
  "Splits STR by DELIM."
  (multiple-value-bind (res)
      (split-sequence:split-sequence delim str)
    res))

(defun trim-string (str)
  "Removes leading and trailing whitespace from STR."
  (string-trim '(#\Space #\Newline #\Backspace #\Tab #\Linefeed #\Page #\Return #\Rubout) str))

(defun get-num (line)
  "Given a LINE in the form 'KEY, VALUE' extracts VALUE."
  (let ((parts (split-string #\, line)))
    (trim-string (nth 1 parts))))

(defun remove-empty-strings (strings)
  "Removes empty strings from the list of strings STRINGS."
  (remove-if #'(lambda (s) (string= s "")) strings))

(defun remove-ms (str)
  "Removes the last two characters from STR which for our purposes tend
to be trailing 'ms'."
  (subseq str 0 (- (length str) 2)))

(defun get-nums (str)
  "Given STR, the contents of a throughput log generated by the bencher
program, gets all of the throughput measurements taken."
  (let* ((lines (split-string #\Newline str))
         (lines (remove-empty-strings lines))
         (ms (mapcar #'get-num lines))
         (numbers (mapcar #'remove-ms ms))
         (floats (mapcar #'parse-float:parse-float numbers)))
    floats))

(defun get-average-throughput (url)
  "Given URL which is a link to a throughput log generated by the bencher
program computes the average throughput for that log."
  (let ((body (http-get url)))
    (when body
      (let ((nums (get-nums body)))
        (/ (reduce #'+ nums) (length nums))))))

(defun get-average-latency (url)
  "Given URL which is a link to a latency log generated by the bencher
program computes the average latency for that log."
  (let ((body (http-get url)))
    (when body
      (let* ((lines (split-string #\Newline body))
             (lines (remove-empty-strings lines))
             (lines (remove-if-not #'(lambda (s) (str:starts-with? "rtt min/avg/max" s)) lines)))
        (let* ((splits (mapcar #'(lambda (s) (split-string #\/ s)) lines))
               (averages (mapcar #'(lambda (s) (nth 4 s)) splits))
               (floats (mapcar #'parse-float:parse-float averages)))
          (/ (reduce #'+ floats) (length floats)))))))

(defun get-average-tcp-ping-latency (url)
  "Given URL which is a link to a latency log generated by bencher for
TCP pings computes the average latency for that log."
  (let ((body (http-get url)))
    (when body
      (let* ((lines (split-string #\Newline body))
            (lines (remove-empty-strings lines))
            (lines (remove-if-not #'(lambda (s) (str:ends-with? "ms" s)) lines))
            (nums (mapcar #'remove-ms lines))
            (floats (mapcar #'parse-float:parse-float nums)))
        (/ (reduce #'+ floats) (length floats))))))

(defun latency-data-from-lines-tcp (lines)
  (if lines
      (let* ((timestamp (car lines))
	     (latency (car (cdr lines)))
	     (latency (remove-ms latency)))
	(format nil "~a,~a
~a" timestamp latency (latency-data-from-lines-tcp (cddr lines))))
      ""))

(defun get-tcp-latency-data (url)
  (let ((body (http-get url)))
    (when body
      (let* ((lines (split-string #\Newline body))
	     (lines (remove-empty-strings lines)))
	(format nil "timestamp,latency
~a" (latency-data-from-lines-tcp lines))))))

(defun latency-data-from-lines (lines)
  (if lines
    (let ((timestamp (car lines))
	  (resline (car (cddr lines))))
      (let* ((split (split-string #\/ resline))
	     (average (nth 4 split)))
	(str:join "
" (list (format nil "~a,~a" timestamp average)
	(latency-data-from-lines (cdddr lines))))))
    ""))

(defun get-latency-data (url)
  (let ((body (http-get url)))
    (when body
      (let* ((lines (split-string #\Newline body))
	     (lines (remove-empty-strings lines)))
	(format nil "timestamp,latency
~a" (latency-data-from-lines lines))))))

(defun request-log-to-csv (url)
  "Converts the throughput request log for an instance to a csv and
returns it as a string."
  (let ((body (http-get url)))
    (str:join "
" (list
   "timestamp,time"
   (str:replace-all ", " "," body)))))


(defun output-to-file (filename fn &rest args)
  "Invokes FN with ARGS and stores the result in a new file called
FILENAME."
  (with-open-file (str filename
                     :direction :output
                     :if-exists :supersede
                     :if-does-not-exist :create)
    (format str (apply fn args))))
