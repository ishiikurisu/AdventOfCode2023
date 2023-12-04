(ns advent-of-code.day3)

(defn re-seq-pos [pattern string]
	(let [m (re-matcher pattern string)]
		((fn step []
			 (when (. m find)
				 (cons {:start (. m start) :end (. m end) :group (. m group)}
							 (lazy-seq (step))))))))

(defn read-input []
	(loop [n 0
				 inlet []]
		(let [line (read-line)]
			(if (-> line count pos?)
				(recur (inc n)
							 (conj inlet line))
				inlet))))

(defn within-gear [part gear]
  (let [left-boundary (-> gear (get :start) dec)
        right-boundary (-> gear (get :end) inc)
        part-start (get part :start)
        part-end (get part :end)]
    (or (and (< left-boundary part-start) (> right-boundary part-start))
        (and (< left-boundary part-end) (> right-boundary part-end))
        (and (< part-start left-boundary) (> part-end right-boundary)))))

(defn get-nearby-parts [gear parts-by-line line-number line-count]
  (let [within-gear-f #(within-gear % gear)
        previous-line-parts (if (pos? line-number)
                              (filter within-gear-f
                                      (nth parts-by-line (dec line-number)))
                              [])
        current-line-parts (filter within-gear-f
                                   (nth parts-by-line line-number))
        next-line-parts (if (< (inc line-number) line-count)
                          (filter within-gear-f
                                  (nth parts-by-line (inc line-number)))
                          [])]
    (flatten (conj previous-line-parts
                   current-line-parts
                   next-line-parts))))

(defn spy [it]
  (println it)
  it)

(defn evaluate-input [inlet]
  (let [parts-by-line (map #(re-seq-pos #"\d+" %) inlet)
        gears-by-line (map #(re-seq-pos #"\*" %) inlet)
        line-count (count inlet)]
    (->> gears-by-line
         (map-indexed (fn [line-number gears]
                        (map (fn [gear]
                               (assoc gear
                                      :parts
                                      (get-nearby-parts gear
                                                        parts-by-line
                                                        line-number
                                                        line-count))) 
                             gears)))
         flatten
         (map #(get % :parts))
         (filter #(= 2 (count %)))
         (map (fn [parts]
                (map (fn [part]
                       (-> part
                           (get :group)
                           Integer/parseInt))
                     parts)))
         (map #(apply * %))
         (apply +))))

(defn main []
	(let [inlet (read-input)
				outlet (evaluate-input inlet)]
		(println outlet)))

(main)

