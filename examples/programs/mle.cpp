#include <vector>
#include <stdio.h>
using namespace std;
int d[100000000];
int main() {
    for (int i = 1; i < 100000000; i ++) {
        d[i] = d[i-1] ^ (i - 1) & (i + 1);
    }
    int ans = 0;
    for (int i = 100000000; i >= 1; i --) {
        ans ^= ans + d[i - 1];
    }
    printf("%d\n", ans);
    return 0;
}