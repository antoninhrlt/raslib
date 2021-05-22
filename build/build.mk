# This file is part of raslib
# Copyright (c) Antonin Hérault
# Under the MIT license

include "build/project.mk"

SRC = $(shell find src -name '*.cpp')
OBJ = $(SRC:.cpp=.o)


define cxx
	printf "\033[35mCXX\033[0m $1 \033[35m=>\033[0m $2\n"
	$(CXX) $1 -o $2 $3 $(FLAGS)
endef


.PHONY : build
build : $(OUT)


.SILENT : $(OUT)
$(OUT) : $(OBJ)
	$(call cxx,$^,$@,-shared)

.SILENT : $(OBJ)
%.o : %.cpp
	$(call cxx, $<, $@)


.SILENT : install
.PHONY : install
install : $(OUT)
	sudo cp -f $< /usr/lib/
	sudo cp -r include/. /usr/include/raslib
	sudo ldconfig


.PHONY : clean
clean :
	@ rm -f $(OBJ)
