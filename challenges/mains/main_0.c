int atoi(const char *str);
int fprintf( FILE *stream, const char *format, ...);
FILE *fopen(const char *filename, const char *mode);
int fclose(FILE *stream);
int fib(int n);

int main(int argc, const char *argv[]) {
    const int val = atoi(argv[1]);
    const int case = fib(val);

    FILE *f = fopen("test_cases_output.txt", "a");
    if (f) {
        fprintf(f, "%i\n", case);
        fclose(f);
    } else {
        fprintf(stderr, "Failed to write test case.\n");
    }

    return 0;
}