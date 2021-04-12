# ******************************************************************************
# Makefile
# ******************************************************************************
# PROJECT : raslib
# LICENSE : MIT
# AUTHOR  : Antonin Hérault
# ******************************************************************************

CXX = g++
FLAGS = -I include -std=c++17 -W -Wall -Wextra -fPIC

SRC = $(shell find src -name '*.cpp')
OBJ = $(SRC:.cpp=.o)

LIB = src/libraslib.so

ARROW = \033[35m=>\033[0m

# ******************************************************************************

define cxx
	@printf "\033[35mCXX\033[0m$(1) $(ARROW)$(2)\n"
	@$(CXX) $(1) -c -o $(2) $(FLAGS)
endef

# ******************************************************************************

# Build the library
makel : $(OBJ)
	@printf "\033[35mCXX\033[0m $^ \n\t$(ARROW) $(LIB)\n"
	@$(CXX) $^ -o $(LIB) -shared $(FLAGS)
	@rm -f $(OBJ)
	@printf "done.\n"

%.o : %.cpp
	$(call cxx, $<, $@)

# ******************************************************************************

# Install the library (and build the library)
install : makel
	@printf "\033[35mCPf\033[0m $(LIB) $(ARROW) /usr/lib/libraslib.so\n"
	@sudo cp -f $(LIB) /usr/lib/
	@printf "\033[35mCPr\033[0m include $(ARROW) /usr/include/raslib/\n"
	@sudo cp -r include/. /usr/include/raslib
	@sudo ldconfig
	@printf "done.\n"
