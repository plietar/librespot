INPUTS=$(wildcard protocol/*.proto)
OUTPUT=pyspotify/protocol

all: $(OUTPUT)/__init__.py pyshn.so

$(OUTPUT)/__init__.py: $(INPUTS)
	protoc --python_out=$(dir $@) --proto_path=protocol $^
	touch $@

pyshn.so:
	cd pyshn && python setup.py build_ext --inplace
	cp pyshn/pyshn.so pyshn.so

.PHONY: all pyshn.so

