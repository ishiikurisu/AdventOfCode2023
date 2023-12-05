(ns advent-of-code.day05
  (:require [clojure.string :as s]))

(defn- mapi [coll f]
  (map f coll))

(defn read-seeds []
  (-> (read-line)
      (s/split #": ")
      second
      (s/split #"\s+")
      (mapi #(Long/parseLong %))))

(defn assign-mapping [inlet destination source length]
  (loop [i 0
         outlet inlet]
    (if (< i length)
      (recur (inc i)
             (assoc outlet (+ i source) (+ i destination)))
      outlet)))

(defn read-mapping []
  (println (read-line))
  (loop [mapping {}]
    (let [line (read-line)
          numbers (when (-> line count pos?)
                    (-> line
                        (s/split #"\s+")
                        (mapi #(Long/parseLong %))))]
      (if (some? numbers)
        (recur (assign-mapping mapping
                               (nth numbers 0)
                               (nth numbers 1)
                               (nth numbers 2)))
        mapping))))

(defn read-mappings []
  (loop [mappings []]
    (let [maybe-mapping (read-mapping)]
      (if (empty? maybe-mapping)
        mappings
        (recur (conj mappings maybe-mapping))))))

(defn apply-mapping [mappings seed]
  (reduce (fn [location mapping]
            (get mapping location location))
          seed
          mappings))

(defn main []
  (let [seeds (read-seeds)
        _ (read-line)
        mappings (read-mappings)
        locations (map #(apply-mapping mappings %) seeds)]
    (println (apply min locations))))

(main)

