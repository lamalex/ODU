random_year(){
	FLOOR=12
	RANGE=17
	year=0   #initialize
	while [ "$number" -le $FLOOR ]
	do
  	year=$RANDOM
  	let "year %= $RANGE"  # Scales $number down within $RANGE.
	done
}

random_year
seq 20${year} 20${year}
