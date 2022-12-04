create:
	python create_puzzle.py

create-specific:
	python create_puzzle.py --year=$(year)

run:
	cd $(year); cargo run --example puzzle_$(puzzle)

test:
	cd $(year); cargo test --example puzzle_$(puzzle)