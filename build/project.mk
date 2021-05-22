# This file is part of raslib
# Copyright (c) Antonin Hérault
# Under the MIT license

CXX = g++
VERSION = c++17
FLAGS = -I include -std=$(VERSION) -W -Wall -Wextra -fPIC
OUT = out/libraslib.so
