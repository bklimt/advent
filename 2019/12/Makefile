
all: a.out

a.out: code.f
	gfortran-8 -fimplicit-none -o $@ $^

run: a.out
	./a.out
