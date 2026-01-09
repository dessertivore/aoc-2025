touch: 
	solve_day

DAY ?= 1

solve_day:
	cargo run --bin day_$(DAY)