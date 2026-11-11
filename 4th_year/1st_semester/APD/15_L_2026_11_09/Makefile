CXX := clang++
CXXFLAGS := -std=c++20 -Wall -Wextra -g -O0 -fopenmp

SRC := $(wildcard *.cpp)
TARGET := main

.PHONY: all run clean

all: $(TARGET)

$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) $(SRC) -o $(TARGET)

run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)
