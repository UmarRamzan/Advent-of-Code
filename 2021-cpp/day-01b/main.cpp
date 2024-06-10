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
    for (int i = 0; i < arr.size()-2; i++) {
        int curr_window = arr[i] + arr[i+1] + arr[i+2];
        int next_window = arr[i+1] + arr[i+2] + arr[i+3];
        if (curr_window < next_window) {
            num_increases++;
        }
    }

    inputFile.close();

    cout << "Solution: " << num_increases << "\n";

}