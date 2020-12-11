
all: bin/problem1 bin/problem2

bin/problem1: obj/problem1.o
	gcc -o bin/problem1 obj/problem1.o -lstdc++

obj/problem1.o: src/problem1.cc
	gcc -o obj/problem1.o -c src/problem1.cc

bin/problem2: obj/problem2.o
	gcc -o bin/problem2 obj/problem2.o -lstdc++

obj/problem2.o: src/problem2.cc
	gcc -o obj/problem2.o -c src/problem2.cc
