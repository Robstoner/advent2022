#include <iostream>
#include <fstream>

std::ifstream fin("input.txt");

int s_max = 0, nr = 0;

int main()
{
    std::string line;
    int s = 0, nr_elves = 1;
    while (std::getline(fin, line))
    {
        //std::cout << line[0] << std::endl;
        if (line.size() == 0)
        {
        //    std:: cout << "s = " << s << std::endl;
            if (s > s_max)
            {
                nr = nr_elves;
                s_max = s;
            }
            s = 0;
            nr_elves++;
        }
        else
        {
            s += atoi(line.c_str());
        }
    }

    std::cout << s_max << " " << nr;
}