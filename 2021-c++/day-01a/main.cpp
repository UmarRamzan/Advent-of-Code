#include <iostream>
#include <fstream>
#include <vector>
using namespace std;

int main() {
    string inputText;

    vector<int> arr;

    ifstream inputFile("input.txt");
    while (getline (inputFile, inputText)) {
        arr.push_back(stoi(inputText));
    }

    int num_increases = 0;
    for (int i = 1; i < arr.size(); i++) {
        if (arr[i] > arr[i-1]) {
            num_increases++;
        }
    }

    inputFile.close();

    cout << "Solution: " << num_increases << "\n";

}