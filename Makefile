
define compile_algorithm
	snowball ./algorithms/$(1).sbl -rust -o ./src/snowball/algorithms/$(1)

	if sed --version 2>/dev/null | grep -q GNU; then \
		sed -i "s/use snowball::SnowballEnv;/use super::super::snowball_env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	else \
		sed -i "" "s/use snowball::SnowballEnv;/use super::super::snowball_env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "" "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	fi

	$(eval FEAT_NAME := $(shell echo "$(1)" | sed -e 's/_/-/g'))

	echo "\n// $(2)\n\n#[cfg(feature = \"$(FEAT_NAME)\")]\nmod $(1);\n" >> ./src/snowball/algorithms/mod.rs
	echo "#[cfg(feature = \"$(FEAT_NAME)\")]\npub fn $(1)(env: &mut SnowballEnv) -> bool {\n    return $(1)::stem(env);\n}\n" >> ./src/snowball/algorithms/mod.rs
endef

.PHONY: default algorithms

default:

algorithms:
	rm -rf ./src/snowball/algorithms
	mkdir ./src/snowball/algorithms

	echo "\nuse super::snowball_env::SnowballEnv;\n\npub type Algorithm = fn(&mut SnowballEnv) -> bool;\n" >> ./src/snowball/algorithms/mod.rs

	$(call compile_algorithm,arabic,Arabic)
	$(call compile_algorithm,armenian_mkrtchyan,ArmenianMkrtchyan)
	$(call compile_algorithm,basque,Basque)
	$(call compile_algorithm,catalan,Catalan)
	$(call compile_algorithm,czech_dolamic_aggressive,CzechDolamicAggressive)
	$(call compile_algorithm,czech_dolamic_light,CzechDolamicLight)
	$(call compile_algorithm,danish,Danish)
	$(call compile_algorithm,dutch,Dutch)
	$(call compile_algorithm,english_lovins,EnglishLovins)
	$(call compile_algorithm,english_porter,EnglishPorter)
	$(call compile_algorithm,english_porter_2,EnglishPorter2)
	$(call compile_algorithm,estonian_freienthal,EstonianFreienthal)
	$(call compile_algorithm,finnish,Finnish)
	$(call compile_algorithm,french,French)
	$(call compile_algorithm,german,German)
	$(call compile_algorithm,greek,Greek)
	$(call compile_algorithm,hindi_lightweight,HindiLightweight)
	$(call compile_algorithm,hungarian,Hungarian)
	$(call compile_algorithm,indonesian_tala,IndonesianTala)
	$(call compile_algorithm,irish_gaelic,IrishGaelic)
	$(call compile_algorithm,italian,Italian)
	$(call compile_algorithm,lithuanian_jocas,LithuanianJocas)
	$(call compile_algorithm,nepali,Nepali)
	$(call compile_algorithm,norwegian_bokmal,NorwegianBokmal)
	$(call compile_algorithm,portuguese,Portuguese)
	$(call compile_algorithm,romanian_heidelberg,RomanianHeidelberg)
	$(call compile_algorithm,romanian_tirdea,RomanianTirdea)
	$(call compile_algorithm,romanian,Romanian)
	$(call compile_algorithm,russian,Russian)
	$(call compile_algorithm,spanish,Spanish)
	$(call compile_algorithm,swedish,Swedish)
	$(call compile_algorithm,turkish_cilden,TurkishCilden)
	$(call compile_algorithm,yiddish_urieli,YiddishUrieli)
