#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

std::ifstream fin("input.txt");

std::vector<int> s_max(3, 0), nr(3, 0);

void check(int s, int nr_elves)
{
    for (int i = 0; i < 3; ++i)
    {
        // std::cout << "s = " << s << " s_max = " << s_max[i] << std::endl;
        if (s > s_max[i] && std::find(nr.begin(), nr.end(), nr_elves) == nr.end())
        {
            for (int j = 1; j >= i; --j)
            {
                s_max[j + 1] = s_max[j];
                nr[j + 1] = nr[j];
            }

            // for (int j = 0; j < 3; ++j)
            // {
            //     std::cout << s_max[j] << " " << nr[j] << std::endl;
            // }

            nr[i] = nr_elves;
            s_max[i] = s;
            break;
        }
    }
}

int main()
{
    std::string line;
    int s = 0, nr_elves = 1;
    while (std::getline(fin, line))
    {
        // std::cout << line[0] << std::endl;
        if (line.size() == 0)
        {
            // std::cout << "empty line";
            // for (int i = 0; i < 3; ++i)
            // {
            //     std::cout << s_max[i] << " " << nr[i] << std::endl;
            // }
            //    std:: cout << "s = " << s << std::endl;
            check(s, nr_elves);
            s = 0;
            nr_elves++;
        }
        else
        {
            s += atoi(line.c_str());
        }
    }

    check(s, nr_elves);

    int sum = 0;
    for (int i = 0; i < 3; ++i)
    {
        sum += s_max[i];
        std::cout << s_max[i] << " " << nr[i] << std::endl;
    }

    std::cout << sum;
}