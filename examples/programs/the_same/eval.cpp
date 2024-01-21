#include <cstdio>
using namespace std;

int main() {
    long long test_out_size;
    fread(&test_out_size, 1, sizeof(test_out_size), stdin);
    char test_out[test_out_size];
    fread(test_out, 1, test_out_size, stdin);


    long long std_out_size;
    fread(&std_out_size, 1, sizeof(std_out_size), stdin);
    char std_out[std_out_size];
    fread(std_out, 1, std_out_size, stdin);

    if (test_out_size != std_out_size) {
        printf("WA");
        return 0;
    }

    for (int i = 0; i < test_out_size; i ++) {
        if (test_out[i] != std_out[i]) {
            printf("WA");
            return 0;
        }
    }
    printf("AC");
    return 0;
}