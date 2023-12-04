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

(defn has-component? [stuff]
	(if (-> stuff count pos?)
		(-> stuff
				count
				pos?)
		false))

(defn evaluate-match [match line-number inlet]
	(println "---")
	(let [start (get match :start)
				end (get match :end)
				line-length (-> inlet first count)
				north-string (if (pos? line-number)
											 (-> (nth inlet (dec line-number))
													 (subs (max 0 (dec start))
																 (min (inc end) line-length)))
											 "")
				south-string (if (< (inc line-number) (count inlet))
											 (-> (nth inlet (inc line-number))
													 (subs (max 0 (dec start))
																 (min (inc  end) line-length)))
											 "")
				west-string (if (-> start dec pos?)
											(-> (nth inlet line-number)
													(subs (dec start)
																start))
											"")
				east-string (if (-> end inc (< line-length))
											(-> (nth inlet line-number)
													(subs end
																(inc end)))
											"")
				_ (println north-string)
				_ (println south-string)
				_ (println west-string)
				_ (println east-string)
				has-north-component (has-component? north-string)
				has-south-component (has-component? south-string)
				has-east-component (has-component? east-string)
				has-west-component (has-component? west-string)
				is-component (or has-north-component
												 has-south-component
												 has-east-component
												 has-west-component)]
		(assoc match :is-component is-component)))

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
			 (filter #(get % :is-component))
			 (map #(get % :group))
			 (map #(Integer/parseInt %))
			 (apply +)))

(defn main []
	(let [inlet (read-input)
				outlet (evaluate-input inlet)]
		(println outlet)
		))

(main)

