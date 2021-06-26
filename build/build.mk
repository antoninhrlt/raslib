# This file is part of raslib
# Copyright (c) Antonin Hérault
# Under the MIT license

include build/project.mk

SRC = $(shell find src -name '*.cpp')
OBJ = $(patsubst src/%.cpp,out/%.o,$(SRC))

.SILENT : init
init :
	mkdir -p out
	mkdir -p out/pin
	mkdir -p out/tcp
	mkdir -p out/uti

build : init $(OUT)

.SILENT : $(OUT)
$(OUT) : $(OBJ)
	- echo "CXX $^ -> $@"
	$(CXX) -shared $^ -o $@ $(FLAGS)

.SILENT : $(OBJ)
out/%.o : src/%.cpp
	- echo "CXX $< -> $@"
	$(CXX) -c $< -o $@ $(FLAGS)

.SILENT : install
.PHONY : install
install : build
	sudo cp -f $(OUT) /usr/lib/
	sudo cp -r include/. /usr/include/raslib
	sudo ldconfig
	- echo "done."

.PHONY : clean
clean :
	@ rm -f $(OBJ)
