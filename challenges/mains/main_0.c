#include <stdio.h>
int fib(int n);

int main(int argc, const char *argv[]) {
    int val;
    
    if (sscanf(argv[1], "%d", &val) != 1) {
        fprintf(stderr, "Failed to parse argv.\n");
        return 1;
    }

    const int result = fib(val);

    FILE *f = fopen("/client/test_cases_output.txt", "a");
    if (f) {
        fprintf(f, "%i\n", result);
        fclose(f);
    } else {
        fprintf(stderr, "Failed to write test case.\n");
        return 1;
    }

    return 0;
}