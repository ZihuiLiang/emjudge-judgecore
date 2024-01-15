#include <algorithm>
#include <cstdio>
#include <iostream>
#include <fstream>
using namespace std;
int main() {
    int l, r, x;
    ifstream interactor_in("interactorin");
    ofstream interactor_out("interactorout");
    interactor_in >> l >> r >> x;
    printf("%d %d\n", l, r);
    fflush(stdout);
    int steps = 0;
    while (1) {
        char c;
        int y;
        scanf(" %c %d", &c, &y);
        steps ++;
        if (y < l || y > r) {
            printf("Error\n");
            fflush(stdout);
            continue;
        }
        if (c == '?') {
            if (y < x) {
                printf("L\n");
            } 
            if (y > x) {
                printf("R\n");
            }
            if (y == x) {
                printf("E\n");
            }
            fflush(stdout);
            continue;
        }
        if (c == '!') {
            if (y == x) {
                interactor_out << "AC with "<<  steps << " steps\n";
            } else {
                interactor_out << "WA with "<<  steps << " steps\n";
            }
            break;
        }
        printf("Error\n");
        fflush(stdout);
    }
    interactor_in.close();
    interactor_out.close();
    return 0;
}