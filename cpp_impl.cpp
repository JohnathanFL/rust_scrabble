// cpp_impl.cpp: The original (and terribly written) C++ implementation 
// for scrabble. Copied from the original scrabble project and used as 
// a basis for the much improved rust_scrabble.

#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

using namespace std;

// How the hell isn't this a standard func by now?
void clearScreen() {
#ifdef __WIN32__
  system("cls");
#endif

#ifdef __linux__
  system("clear");
#endif
}

void showMenu();

bool checkCanMakeWord(string desired, string scrabbled, bool output);
void checkWildcard(string desired, string scrabbled);
void findLongestWord(string currentTiles, const vector<string>& wordList);

int main(int argc, char* argv[]) {
  ifstream inFile("wordList.txt");
  vector<string> wordList;
  string choice, desired, scrabbled;

  string temp;
  while (getline(inFile, temp))
    wordList.push_back(temp);

  clearScreen();
  showMenu();

  while (getline(cin, choice) && choice != "") {


    switch (choice[0] - '0') {
    case 1:
      cout << "Enter a desired word: ";
      cin >> desired;

      cout << "Enter posessed tiles: ";
      cin >> scrabbled;
      checkCanMakeWord(desired, scrabbled, true);
      break;

    case 2:
      cout << "Enter current word, using ?s as wildcards: ";
      cin >> scrabbled;

      cout << "Enter desired word: ";
      cin >> desired;
      checkWildcard(desired, scrabbled);
      break;

    case 3:
      cout << "Enter current tiles: ";
      cin >> scrabbled;
      findLongestWord(scrabbled, wordList);
      break;

    default:
      cout << "Invalid choice!\n";
    }

    clearScreen();
    showMenu();
  }

  return 0;
}

void showMenu() {
  cout <<
      R"(
1.    Check if a word can be made from tiles.
2.    Check words that can be made from tiles and wildcards.
3.    Find the longest word that can be made from given tiles.
(Or simply press enter to quit)
====>)";
}

bool checkCanMakeWord(string desired, string scrabbled, bool output) {
  map<char, int> desLetterCounts, scrabLetterCounts;
  bool canMakeWord = true;

  for (char curChar : desired) {
    curChar = tolower(curChar);

    if (desLetterCounts.find(curChar) == desLetterCounts.end())
      desLetterCounts[curChar] = 1;
    else
      desLetterCounts[curChar]++;
  }

  for (char curChar : scrabbled) {
    curChar = tolower(curChar);

    if (scrabLetterCounts.find(curChar) == scrabLetterCounts.end())
      scrabLetterCounts[curChar] = 1;
    else
      scrabLetterCounts[curChar]++;
  }

  for (pair<char, int> keyPair : desLetterCounts)
    // If the letter's count in the letter pool is less, we can't make the word.
    if (scrabLetterCounts[keyPair.first] < keyPair.second)
      canMakeWord = false;

  if (output) {
    if (canMakeWord)
      cout << "Can make word.";
    else
      cout << "Cannot make word.";

    cin.ignore();
    cin.get();
  }
  return canMakeWord;
}
void checkWildcard(string desired, string scrabbled) {
  // If the two words aren't the same length, they can't possibly be the same
  // word.
  bool isSame = desired.length() == scrabbled.length();


  for (int i = 0; i <= scrabbled.length() && isSame; i++)
    if (scrabbled[i] != '?')          // No reason to test wildcards,
      if (scrabbled[i] != desired[i]) // since they could be anything.
        isSame = false;

  cout << "Desired word " << (isSame ? "can" : "cannot ")
       << "be made from insertions to scrabbled.";
}
void findLongestWord(string currentTiles, const vector<string>& wordList) {
  clearScreen();
  string line, currentLongest = "";
  for (string line : wordList) {
    if (line[line.length() - 1] == 13)
      line = line.substr(0, line.length() - 1);


    if (checkCanMakeWord(line, currentTiles, false)) {
      if (line.length() > currentLongest.length())
        currentLongest = line;
    }
  }

  cout << "Longest possible word is: " << currentLongest << endl;
  cin.ignore();
  cin.get();
}
