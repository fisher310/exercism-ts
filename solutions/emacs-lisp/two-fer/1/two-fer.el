(defun two-fer (&optional name)
  "the two-fer function"
  (if name
      (message (concat "One for " name ", one for me."))
    (message "One for you, one for me.")))

(provide 'two-fer)

