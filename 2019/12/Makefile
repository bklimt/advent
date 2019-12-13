
all: bin/a.out

clean:
	rm -rf ./bin/

bin/a.out: code.f
	mkdir -p bin && gfortran-8 -fimplicit-none -o $@ $^

run: bin/a.out
	./bin/a.out
