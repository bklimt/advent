
#include <memory.h>
#include <stdio.h>
#include <stdlib.h>

void phase(int *in, int *out, long n) {
  for (int i = 0; i < n; i++) {
    if (i > n/2) {
      out[i] = out[i-1] - in[i-1];
      continue;
    }

    out[i] = 0;
    int s = 1;
    long j = i;
    while (j < n) {
      // Add the ones that are non-zero.
      long end = j + i + 1;
      if (end > n) {
        end = n;
      }
      for (int k = j; k < end; k++) {
        out[i] += s * in[k];
      }
      // Skip the ones that are zero.
      j = end + i + 1;
      s *= -1;
    }
  }
  for (int i = 0; i < n; i++) {
    out[i] = abs(out[i]) % 10;
  }
}

void phase2(int *in, int *out, long n) {
  int sum = 0;
  for (int i = n-1; i > (n/2); i--) {
    sum += in[i];
    out[i] = abs(sum) % 10;
  }
}

long read_file_size(const char *path) {
  FILE *f = fopen(path, "rb");
  fseek(f, 0, SEEK_END);
  long size = ftell(f);
  fclose(f);
  return size;
}

void read_input(int *buf, const char *path, long n) {
  FILE *f = fopen(path, "rb");
  for (long i = 0; i < n; i++) {
    int c = fgetc(f);
    if (c == EOF) {
      fprintf(stderr, "unexpected end of file\n");
      exit(-1);
    }
    buf[i] = c-'0';
  }
  fclose(f);
}

// Copies the first n bytes of the buffer m times.
void dup_buffer(int *buf, long n, int m) {
  for (int i = 1; i < m; i++) {
    memcpy(buf + n * i, buf, n * sizeof(int));
  }
}

void print_buffer(int *buf, long n) {
  for (long i = 0; i < n; i++) {
    printf("%c", buf[i] + '0');
  }
  printf("\n");
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "usage: ./a.out input.txt\n");
    exit(-1);
  }

  int dup_count = 10000;

  const char *path = argv[1];
  long fs = read_file_size(path);
  printf("fs = %ld\n", fs);
  long n = fs * dup_count;
  printf("n = %ld\n", n);

  int *buf1 = malloc(n * sizeof(int));
  int *buf2 = malloc(n * sizeof(int));

  read_input(buf1, path, fs);
  dup_buffer(buf1, fs, dup_count);

  print_buffer(buf1, fs * 3);
  
  // 16.2:
  // We actually only care about offset 5970443.
  // The input length is 6500000.
  // The offset we care about is more than halfway down the matrix...

  for (int i = 0; i < 100; i++) {
    printf("phase %d...\n", i);
    phase2(buf1, buf2, n);
    int *tmp = buf1;
    buf1 = buf2;
    buf2 = tmp;
  }
  //print_buffer(buf1, n);
  print_buffer(buf1+5970443, 8);
  //print_buffer(buf1+303673, 8);

  return 0;
}
