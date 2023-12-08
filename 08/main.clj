(ns advent-of-code.day8
  (:require [clojure.string :as s]))

(defn read-graph []
  (loop [graph {}]
    (let [line (read-line)]
      (if (-> line count pos?)
        (let [fields (s/split line #"\W+")]
          (recur (assoc graph
                        (nth fields 0) 
                        [(nth fields 1) (nth fields 2)])))
        graph))))

(defn traverse-graph [commands graph from outlet]
  (if (= "ZZZ" from)
    (* outlet (count commands))
    (let [to (reduce (fn [where command]
                       (let [options (get graph where)]
                         (if (= \L command)
                           (first options)
                           (second options))))
                     from
                     commands)]
      (traverse-graph commands
                      graph
                      to
                      (inc outlet)))))

(defn main []
  (let [commands (read-line)
        _ (read-line)
        graph (read-graph)]
    (println (traverse-graph commands graph "AAA" 0))))

(main)

