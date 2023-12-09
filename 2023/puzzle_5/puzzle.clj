(ns clojure.examples.puzzle
  (:gen-class))

(require '[clojure.string :as str])

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

(defn read-lines [file-path]
  (with-open [reader (clojure.java.io/reader file-path)]
  (doall (line-seq reader)))
)

(defn split-and-parse-ints [values]
  (map #(bigint %) (str/split values (re-pattern (str "\\" " "))))
)

(defn single-to-interval [seed]
  [seed seed])

(defn pair-to-interval [seeds]
  (let [
    start (first seeds)
    length (second seeds)
  ]
  [start (- (+ start length) 1)]))

(defn part-1-range-parse [line]
  (let [sliced (subs line 7)
    raw-seeds (split-and-parse-ints sliced)]
    (map single-to-interval raw-seeds))
)

(defn part-2-range-parse [line]
  (let [sliced (subs line 7)
    raw-seeds (split-and-parse-ints sliced)]
    (map pair-to-interval (partition 2 2 raw-seeds)))
)

(defn add-translation-range [translation-layer line]
  (let [
    [dest-start source-start length] (split-and-parse-ints line)
    translation-range [[source-start (- (+ source-start length) 1)] [dest-start (- (+ dest-start length) 1)]]
  ]
  (conj translation-layer translation-range))
)

(defn parse-line [current-layers line]
   (cond
     (empty? line) current-layers
     (str/includes? line "map") (conj current-layers [])
     :else (conj (vec (butlast current-layers)) (add-translation-range (last current-layers) line)))
)

; Parse translations tables, making sure source ranges are in ascending order
(defn parse-translations [lines]
  (let [
    translations (reduce (fn [current-layers line] (parse-line current-layers line)) [] lines)
  ]
  (map (partial sort-by first) translations))
)


(defn solve-puzzle []
   (def example (read-lines "puzzle_5/example.txt"))
   (println (part-1-range-parse (first example)))
   (println (part-2-range-parse (first example)))
   (println(parse-translations (rest example)))

   ; Should be 35
   (println (get-min-seed (parse-translations (rest example)) (part-1-range-parse (first example))))
   ; Should be 46
   (println (get-min-seed (parse-translations (rest example)) (part-2-range-parse (first example))))

   (def input (read-lines "puzzle_5/input.txt"))
   (println (get-min-seed (parse-translations (rest input)) (part-1-range-parse (first input))))
   (println (get-min-seed (parse-translations (rest input)) (part-2-range-parse (first input))))
)(solve-puzzle)