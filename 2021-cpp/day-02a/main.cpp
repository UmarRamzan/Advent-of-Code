#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
using namespace std;

vector<string> split(const string &str, char delimiter) {
    vector<string> tokens;
    string token;
    istringstream tokenStream(str);

    while (getline(tokenStream, token, delimiter)) {
        tokens.push_back(token);
    }

    return tokens;
}

int main() {
    string inputText;
    ifstream inputFile("input.txt");

    int pos_x = 0;
    int pos_y = 0;

    while (getline (inputFile, inputText)) {
        vector<string> elems = split(inputText, ' ');

        int mov_value = stoi(elems[1]);

        if (elems[0] == "forward") {pos_x += mov_value;}
        else if (elems[0] == "down") {pos_y += mov_value;}
        else if (elems[0] == "up") {pos_y -= mov_value;}
        
    }

    inputFile.close();

    cout << "Solution: " << pos_x * pos_y << endl;

}