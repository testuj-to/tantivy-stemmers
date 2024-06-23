
define compile_algorithm
	snowball ./algorithms/$(1).sbl -rust -o ./src/snowball/algorithms/$(1)

	if sed --version 2>/dev/null | grep -q GNU; then \
		sed -i "s/use snowball::SnowballEnv;/use super::super::env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	else \
		sed -i "" "s/use snowball::SnowballEnv;/use super::super::env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "" "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	fi

	echo "\n// $(1)\n\n#[cfg(feature = \"$(1)\")]\nmod $(1);\n" >> ./src/snowball/algorithms/mod.rs
	echo "#[cfg(feature = \"$(1)\")]\npub fn $(1)(env: &mut SnowballEnv) -> bool {\n    $(1)::stem(env)\n}\n" >> ./src/snowball/algorithms/mod.rs
endef

.PHONY: default algorithms test test-with-print

default:

algorithms:
	rm -rf ./src/snowball/algorithms
	mkdir ./src/snowball/algorithms

	echo "use super::env::SnowballEnv;\n" >> ./src/snowball/algorithms/mod.rs

	$(call compile_algorithm,arabic)
	$(call compile_algorithm,armenian_mkrtchyan)
	$(call compile_algorithm,basque)
	$(call compile_algorithm,catalan)
	$(call compile_algorithm,czech_dolamic_aggressive)
	$(call compile_algorithm,czech_dolamic_light)
	$(call compile_algorithm,danish)
	$(call compile_algorithm,dutch)
	$(call compile_algorithm,english_lovins)
	$(call compile_algorithm,english_porter)
	$(call compile_algorithm,english_porter_2)
	$(call compile_algorithm,estonian_freienthal)
	$(call compile_algorithm,finnish)
	$(call compile_algorithm,french)
	$(call compile_algorithm,german)
	$(call compile_algorithm,greek)
	$(call compile_algorithm,hindi_lightweight)
	$(call compile_algorithm,hungarian)
	$(call compile_algorithm,indonesian_tala)
	$(call compile_algorithm,irish_gaelic)
	$(call compile_algorithm,italian)
	$(call compile_algorithm,lithuanian_jocas)
	$(call compile_algorithm,nepali)
	$(call compile_algorithm,norwegian_bokmal)
	$(call compile_algorithm,portuguese)
	$(call compile_algorithm,romanian_heidelberg)
	$(call compile_algorithm,romanian_tirdea)
	$(call compile_algorithm,romanian)
	$(call compile_algorithm,russian)
	$(call compile_algorithm,spanish)
	$(call compile_algorithm,swedish)
	$(call compile_algorithm,turkish_cilden)
	$(call compile_algorithm,yiddish_urieli)

test:
	cargo test --all-features

test-with-print:
	cargo test --all-features -- --nocapture
