#lang racket

(define (status->enum s)
  (cond
    [(string=? s "C") "Mapping::Common"]
    [(string=? s "F") "Mapping::Full"]
    [(string=? s "S") "Mapping::Simple"]
    [(string=? s "T") "Mapping::Turkic"]))

(define (codepoints->char-list cps)
  (format "&[~a]" (string-join (map codepoint->char cps) ", ")))

(define (codepoint->char cp)
  (format "'\\u{~a}'" cp))

(define (racket->rust input-list)
  (display "let data: &[(char, Mapping, &[char])] = &[\n")
  (for-each
   (λ (set)
     (display
      (format "    (~a, ~a, ~a),\n"
              (codepoint->char (vector-ref set 0))
              (status->enum (vector-ref set 1))
              (let ((chars (vector-ref set 2)))
                (if (list? chars)
                    (codepoints->char-list chars)
                    (codepoints->char-list (list chars)))))))
   input-list)
  (display "];\n"))

(define (filter-file name)
  (filter-map
   (λ (line)
     (if (and
          (non-empty-string? (string-trim line #:repeat? #t))
          (not (string-prefix? line "#")))
         line
         #f))
   (file->lines name)))

(define (split-case-file lines)
  (map
   (λ (line)
     (list->vector
      (filter-map
       (λ (str)
         (let ([field (string-trim str #:repeat? #t)])
           (if (string-prefix? field "#")
               #f
               (if (string-contains? field " ")
                   (string-split field " " #:repeat? #t)
                   field))))
       (string-split line ";"))))
   lines))

(racket->rust
 (split-case-file
  (filter-file "CaseFolding.txt")))