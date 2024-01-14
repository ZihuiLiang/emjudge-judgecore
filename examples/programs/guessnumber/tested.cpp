#include <algorithm>
#include <cstdio>
#include <iostream>
using namespace std;
int main() {
    int l, r;
    scanf("%d %d", &l, &r);
    while (1) {
        printf("? %d\n", l);
        fflush(stdout);
        char c;
        scanf(" %c", &c);
        if (c == 'L') {
            l ++;
        }
        if (c == 'E') {
            printf("! %d\n", l);
            fflush(stdout);
            break;
        }
    }
    return 0;
}