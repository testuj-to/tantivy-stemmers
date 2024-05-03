
define compile_algorithm
	snowball ./algorithms/$(1).sbl -rust -o ./src/snowball/algorithms/$(1)

	if sed --version 2>/dev/null | grep -q GNU; then \
		sed -i "s/use snowball::SnowballEnv;/use super::super::snowball_env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	else \
		sed -i "" "s/use snowball::SnowballEnv;/use super::super::snowball_env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "" "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	fi

	echo "pub mod $(1);" >> ./src/snowball/algorithms/mod.rs
endef

.PHONY: default algorithms

default:

algorithms:
	rm -rf ./src/snowball/algorithms
	mkdir ./src/snowball/algorithms

	$(call compile_algorithm,czech_dolamic_aggressive)
	$(call compile_algorithm,czech_dolamic_light)
	$(call compile_algorithm,english_porter)
# $(call compile_algorithm,english_porter_2)
	$(call compile_algorithm,french)
	$(call compile_algorithm,german)
