;;; acronym.el --- Acronym (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:


(defun acronym (phrase)
  (upcase
   (mapconcat #'identity
              (mapcar (lambda(s) (substring s 0 1))
                      (split-string phrase " +\\|-" t "_"))
              "")))





(provide 'acronym)
;;; acronym.el ends here
