
all: bin/problem1

bin/problem1: obj/problem1.o
	gcc -std=c++14 -o bin/problem1 obj/problem1.o -lstdc++

obj/problem1.o: src/problem1.cc
	gcc -std=c++14 -o obj/problem1.o -c src/problem1.cc