#include <iostream>
#include <fstream>

int main() {
    std::string filePath = "Cargo.toml";

    std::ifstream inFile(filePath);
    if (inFile.is_open()) {
        std::cout << "File can be read.\n";
        inFile.close(); 
    } else {
        std::cout << "File cannot be read.\n";
    }

    std::ofstream outFile(filePath, std::ios::app);
    if (outFile.is_open()) {
        std::cout << "File can be written.\n";
        outFile.close();
    } else {
        std::cout << "File cannot be written.\n";
    }

    return 0;
}
