
all: bin/problem1 bin/problem2

bin/problem1: obj/problem1.o
	gcc -std=c++14 -o bin/problem1 obj/problem1.o -lstdc++

obj/problem1.o: src/problem1.cc
	gcc -std=c++14 -o obj/problem1.o -c src/problem1.cc

bin/problem2: obj/problem2.o
	gcc -std=c++14 -o bin/problem2 obj/problem2.o -lstdc++

obj/problem2.o: src/problem2.cc
	gcc -std=c++14 -o obj/problem2.o -c src/problem2.cc
