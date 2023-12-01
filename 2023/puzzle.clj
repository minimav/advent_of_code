(ns clojure.examples.puzzle
   (:gen-class))



(defn solve-puzzle []
   (def example (slurp "example.txt"))
   (println example)

   (def input (slurp "input.txt"))
   (println input)
)(solve-puzzle)