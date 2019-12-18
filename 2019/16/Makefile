
all: bin/a.out

clean:
	rm -rf ./bin/

bin/a.out: code.c
	mkdir -p bin && gcc -o $@ $^

run: bin/a.out
	./bin/a.out input.txt
