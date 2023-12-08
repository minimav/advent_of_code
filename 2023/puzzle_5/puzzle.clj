(ns clojure.examples.puzzle
  (:gen-class))

; Intersect two inclusive interger-valued ranges (each of 2 elements)
(defn intersect [range other]
  (if (empty? range) [[] [] []]
    (let [before (if (< (first range) (first other)) [(first range) (min (second range) (- (first other) 1))] [])
      after (if (> (second range) (second other)) [(max (first range) (+ (second other) 1)) (second range)] [])
        intersection (if (or (> (first range) (second other)) (< (second range) (first other))) [] [(max (first range) (first other))  (min (second range) (second other))] )]
         [before intersection after])
  )   
)

(assert (= (intersect [0 2] [2 3]) [[0 1] [2, 2] []]))
(assert (= (intersect [0 1] [2 3]) [[0 1] [] []]))
(assert (= (intersect [0 4] [2 3]) [[0 1] [2 3] [4 4]]))
(assert (= (intersect [3 4] [2 3]) [[] [3 3] [4 4]]))
(assert (= (intersect [] [2 3]) [[] [] []]))

; Translate a range within the source range to a destination range (necessarily of same size)
(defn translate [intersection source dest]
  [
    (+ (first dest) (- (first intersection) (first source)))
    (+ (first dest) (- (second intersection) (first source)))
  ]
)

; Apply a single translation to a set of overlaps
(defn compare-range-to-translation [overlaps range translation]
  (let [
    [source dest] translation
    [before intersection after] (intersect range source)
    new-overlaps (cond-> overlaps 
      (not (empty? before)) (conj before)
      (not (empty? intersection)) (conj (translate intersection source dest)))]
    [new-overlaps after])
)

; Single range translated
(assert (= (compare-range-to-translation [] [0 4] [[1 3] [2 4]]) [[[0 0] [2 4]] [4 4]]))
; Single range translated, updating existing ranges that have been mapped
(assert (= (compare-range-to-translation [[0 1]] [0 4] [[1 3] [2 4]]) [[[0 1] [0 0] [2 4]] [4 4]]))

; Apply a set of translations to a range
(defn compare-range-to-translation-layer [translations range]
  (let [
    [overlaps remaining]
    (reduce (fn [acc translation] (compare-range-to-translation (first acc) (second acc) translation))
      [[] range] translations)
    new-overlaps (cond-> overlaps 
      (not (empty? remaining)) (conj remaining))]
  new-overlaps)
)

; Single range translated via one translation
(assert (= (compare-range-to-translation-layer [[[1 3] [2 4]]] [0 50])
  [[0 0] [2 4] [4 50]])
)
; Single range translated via multiple translations
(assert (= (compare-range-to-translation-layer [[[1 3] [2 4]] [[4 10] [10 16]]] [0 50])
  [[0 0] [2 4] [10 16] [11 50]])
)

(defn compare-ranges-to-translation-layer [translations ranges]
  (mapcat (partial compare-range-to-translation-layer translations) ranges)
)

; Multiple ranges translated via multiple translations
(assert (= (compare-ranges-to-translation-layer [[[1 3] [2 4]] [[4 10] [10 16]]] [[0 50] [0 0]])
  [[0 0] [2 4] [10 16] [11 50] [0 0]])
)

(defn pass-ranges-through-all-translation-layers [translation-layers initial-ranges]
  (reduce (fn [ranges translation-layer] (compare-ranges-to-translation-layer translation-layer ranges))
      initial-ranges translation-layers)
)

(defn get-min-seed [translation-layers initial-ranges]
  (let [
     seed-ranges (pass-ranges-through-all-translation-layers translation-layers initial-ranges)
     ]
  (apply min (map first seed-ranges)))
)

(defn solve-puzzle []
   ;;(def example (slurp "puzzle_5/example.txt"))
   ;;(println example)

   ;(def input (slurp "puzzle_5/input.txt"))
   ;(println input)
)(solve-puzzle)