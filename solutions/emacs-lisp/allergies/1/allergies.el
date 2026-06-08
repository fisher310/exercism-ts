;;; allergies.el --- Allergies Exercise (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:


(defun allergen-list (score)
  (let ((res ()))
    (if (> (logand score 128) 0) (setq res (cons "cats" res)))
    (if (> (logand score 64) 0) (setq res (cons "pollen" res)))
    (if (> (logand score 32) 0) (setq res (cons "chocolate" res)))
    (if (> (logand score 16) 0) (setq res (cons "tomatoes" res)))
    (if (> (logand score 8) 0) (setq res (cons "strawberries" res)))
    (if (> (logand score 4) 0) (setq res (cons "shellfish" res)))
    (if (> (logand score 2) 0) (setq res (cons "peanuts" res)))
    (if (> (logand score 1) 0) (setq res (cons "eggs" res)))
    res))
(allergen-list 3)

(defun allergic-to-p (score allergen)
  (cond
   ((string= "eggs" allergen) (> (logand score 1) 0))
   ((string= "peanuts" allergen) (> (logand score 2) 0))
   ((string= "shellfish" allergen) (> (logand score 4) 0))
   ((string= "strawberries" allergen) (> (logand score 8) 0))
   ((string= "tomatoes" allergen) (> (logand score 16) 0))
   ((string= "chocolate" allergen) (> (logand score 32) 0))
   ((string= "pollen" allergen) (> (logand score 64) 0))
   ((string= "cats" allergen) (> (logand score 128) 0))))


(provide 'allergies)
;;; allergies.el ends here
