#include <cstdio>
using namespace std;

int main() {
    long long input_size;
    fread(&input_size, 1, sizeof(input_size), stdin);
    char input[input_size];
    fread(input, 1, input_size, stdin);

    long long test_out_size;
    fread(&test_out_size, 1, sizeof(test_out_size), stdin);
    char test_out[test_out_size];
    fread(test_out, 1, test_out_size, stdin);


    long long std_out_size;
    fread(&std_out_size, 1, sizeof(std_out_size), stdin);
    char std_out[std_out_size];
    fread(std_out, 1, std_out_size, stdin);

    if (test_out_size && test_out[test_out_size - 1] == '\n') {
        test_out_size --;
    }

    if (std_out_size && std_out[std_out_size - 1] == '\n') {
        std_out_size --;
    }

    if (test_out_size != std_out_size) {
        printf("WA\n");
        return 0;
    }

    for (int i = 0; i < test_out_size; i ++) {
        if (test_out[i] != std_out[i]) {
            printf("WA\n");
            return 0;
        }
    }
    printf("AC\n");
    return 0;
}