#include <iostream>
#include <fstream>
#include <map>
#include <string>
#include <set>
#include <cmath>
using namespace std;

// Trim whitespace from a string
string trimWhitespace(string s)
{
    s.erase(remove(s.begin(), s.end(), ' '), s.end());
    return s;
}

// Parse a space separated list of card strings into a set
set<string> parseCards(string cardsRaw)
{
    set<string> cards;
    size_t splitter = cardsRaw.find(" ");
    while (splitter != string::npos)
    {
        string card = cardsRaw.substr(0, splitter);
        string cleanedCard = trimWhitespace(card);
        if (cleanedCard.length() > 0)
        {
            cards.insert(cleanedCard);
        }
        cardsRaw = cardsRaw.substr(splitter + 1);
        splitter = cardsRaw.find(" ");
    }
    cards.insert(cardsRaw);
    return cards;
}

// Print a set of cards
void printCards(set<string> cards)
{
    for (auto it = cards.begin(); it != cards.end(); ++it)
    {
        cout << *it << ' ';
    }
    cout << '\n';
}

// Read input and solve both parts of the puzzle
void solvePuzzle(string path)
{
    ifstream input;
    string line;
    input.open(path);

    int part1Answer = 0;
    map<int, int> counts;

    if (input.is_open())
    {
        while (input)
        {
            getline(input, line);
            if (line.substr(0, 4) != "Card")
            {
                continue;
            }
            // Remove the initial "Card "
            line = line.substr(5, line.length() - 1);

            size_t colonIndex = line.find(":");
            int gameNumber = stoi(line.substr(0, colonIndex));
            string cards = line.substr(colonIndex + 1);

            size_t verticalLineIndex = cards.find("|");
            string winningRaw = cards.substr(0, verticalLineIndex);
            string handRaw = cards.substr(verticalLineIndex + 1);
            set<string> winning = parseCards(winningRaw);
            set<string> hand = parseCards(handRaw);

            set<string> winningCards;
            set_intersection(
                winning.begin(),
                winning.end(),
                hand.begin(),
                hand.end(),
                inserter(winningCards, winningCards.begin()));

            int numWinningCardsInHand = winningCards.size();
            if (numWinningCardsInHand > 0)
            {
                part1Answer += pow(2, numWinningCardsInHand - 1);
            }
            counts[gameNumber] += 1;
            for (int i = 1; i < numWinningCardsInHand + 1; i++)
            {
                counts[gameNumber + i] += counts[gameNumber];
            }
        }
    }

    // Accumulate traversal counts for part 2 answer
    int part2Answer = 0;
    map<int, int>::iterator it = counts.begin();

    while (it != counts.end())
    {
        part2Answer += it->second;
        ++it;
    }

    printf("\tPart 1: %d\n", part1Answer);
    printf("\tPart 2: %d\n", part2Answer);
}

int main()
{
    cout << "Example:" << '\n';
    solvePuzzle("puzzle_4/example.txt");
    cout << "Input:" << '\n';
    solvePuzzle("puzzle_4/input.txt");
}