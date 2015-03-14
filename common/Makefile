INPUTS=$(wildcard protocol/*.proto)
OUTPUT=pyspotify/protocol

all: $(OUTPUT)/__init__.py

$(OUTPUT)/__init__.py: $(INPUTS)
	protoc --python_out=$(dir $@) --proto_path=protocol $^
	touch $@

