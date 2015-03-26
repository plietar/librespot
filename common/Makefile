INPUTS=$(wildcard protocol/*.proto)
OUTPUT=pylibrespot/protocol

all: $(OUTPUT)/__init__.py pyshn.so pylibrespot/decoder/decoder.so

$(OUTPUT)/__init__.py: $(INPUTS)
	protoc --python_out=$(dir $@) --proto_path=protocol $^
	touch $@

pyshn.so:
	cd pyshn && python setup.py build_ext --inplace
	cp pyshn/pyshn.so $@

pylibrespot/decoder/decoder.so:
	cd pylibrespot/decoder && python setup.py build_ext --inplace

.PHONY: all pyshn.so pylibrespot/decoder/decoder.so

