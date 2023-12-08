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

(defn find-starting-nodes [graph]
  (->> graph
       keys
       (filter #(= \A (nth % 2)))))

(defn found-ending-nodes? [nodes]
  (->> nodes
       (filter #(= \Z (nth % 2)))
       count
       (= (count nodes))))

(defn traverse-graph [commands graph starting-nodes command-size]
  (loop [outlet 0
         nodes starting-nodes]
    (if (found-ending-nodes? nodes)
      outlet
      (recur (inc outlet)
             (map (fn [node]
                    (let [command (nth commands (mod outlet command-size))
                          options (get graph node)]
                      (if (= \L command)
                        (first options)
                        (second options))))
                  nodes)))))

(defn main []
  (let [commands (read-line)
        _ (read-line)
        graph (read-graph)
        starting-nodes (find-starting-nodes graph)
        command-size (count commands)]
    (println (traverse-graph commands graph starting-nodes command-size))
    ))

(main)

