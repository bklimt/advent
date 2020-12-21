
all: bin/problem

bin/problem: obj/problem.o
	gcc -std=c++17 -o bin/problem obj/problem.o -lstdc++

obj/problem.o: src/problem.cc
	gcc -std=c++17 -o obj/problem.o -c src/problem.cc

