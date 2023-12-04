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
							 (conj inlet {:line-number n
														:line line}))
				inlet))))

(defn has-component? [schematic]
	(if (-> schematic count pos?)
    (->> schematic
         (map #(and (not= \. %)
                    (not= \1 %)
                    (not= \2 %)
                    (not= \3 %)
                    (not= \4 %)
                    (not= \5 %)
                    (not= \6 %)
                    (not= \7 %)
                    (not= \8 %)
                    (not= \9 %)
                    (not= \0 %)))
         (every? false?)
         not)
		false))

(comment
  (has-component? ".....21455...")
  (has-component? "...#.21455..."))

(defn evaluate-match [match line-number inlet]
	(let [start (get match :start)
				end (get match :end)
				line-length (-> inlet first (get :line) count)
        left-boundary (max 0 (dec start))
        right-boundary (min line-length (inc end))
				previous-string (if (pos? line-number)
								   			  (-> inlet
                              (nth (dec line-number))
                              (get :line)
													    (subs left-boundary right-boundary))
											    "")
        current-string (-> inlet
                           (nth line-number)
                           (get :line)
                           (subs left-boundary right-boundary))
        next-string (if (< (inc line-number) (count inlet))
                      (-> inlet
                          (nth (inc line-number))
                          (get :line)
                          (subs left-boundary right-boundary))
                      "")
        is-component? (or (has-component? previous-string)
                          (has-component? current-string)
                          (has-component? next-string))]
		(assoc match :is-component? is-component?)))

(defn evaluate-line [sample inlet]
	(let [line-number (:line-number sample)
				line (:line sample)
				matches (re-seq-pos #"\d+" line)]
		(->> matches
				 (map #(evaluate-match % line-number inlet)))))

(defn evaluate-input [inlet]
	(->> inlet
			 (map #(evaluate-line % inlet))
			 flatten
			 (filter #(get % :is-component?))
			 (map #(get % :group))
			 (map #(Integer/parseInt %))
			 (apply +)))

(defn main []
	(let [inlet (read-input)
				outlet (evaluate-input inlet)]
		(println outlet)
		))

(main)

