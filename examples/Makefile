# ******************************************************************************
# Makefile
# ******************************************************************************
# PROJECT : raslib
# LICENSE : MIT
# AUTHOR  : Antonin Hérault
# ******************************************************************************

# ADD ' -lraslib ' !

CXX = g++
FLAGS = -I include -std=c++17 -W -Wall -Wextra -fPIC -pthread \
	-lraslib

FILE = main.cpp

blink : blink/$(FILE)
	$(CXX) $< -o $@/prog $(FLAGS)

motor : motor/$(FILE)
	$(CXX) $< -o $@/prog $(FLAGS)

server : server/$(FILE)
	$(CXX) $< -o $@/prog $(FLAGS)