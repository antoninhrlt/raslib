# This file is part of raslib
# Copyright (c) Antonin Hérault
# Under the MIT license

# * Don't forget to add the library with flag: -lraslib 

CXX = g++
FLAGS = -I include -std=c++17 -W -Wall -Wextra -fPIC -pthread -lraslib

FILE = main.cpp

blink : blink/$(FILE)
	$(CXX) $< -o $@/blink $(FLAGS)

motor : motor/$(FILE)
	$(CXX) $< -o $@/motor $(FLAGS)

server : server/$(FILE)
	$(CXX) $< -o $@/server $(FLAGS)
